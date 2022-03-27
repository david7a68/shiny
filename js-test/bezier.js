'use strict';

let canvas_size = 400;

function lerp(t, x, y) {
    return (1 - t) * x + t * y;
}

function draw_point(canvas, point, color) {
    canvas.beginPath();
    canvas.arc(point[0], canvas_size - point[1], 5, 0, 2 * Math.PI, false);
    canvas.fillStyle = color;
    canvas.fill();
}

class Sample {
    /**
     * @param {Bezier} curve1 
     * @param {Bezier} curve2 
     */
    constructor(curve1, curve2) {
        this.curve1 = curve1;
        this.curve2 = curve2;
    }

    draw(canvas, controls) {
        if (controls) {
            this.curve1.draw_control_points(canvas, 'lightgrey');
            this.curve2.draw_control_points(canvas, 'lightgrey');
        }

        this.curve1.draw(canvas, 'red');
        this.curve2.draw(canvas, 'blue');
    }
}

class Line {
    constructor(a, b, c) {
        let div = Math.sqrt(a * a + b * b);
        this.a = a / div;
        this.b = b / div;
        this.c = c / div;
    }

    y_at(x) {
        return -(this.a * x + this.c) / this.b;
    }

    x_intercept() {
        return -this.c / this.a;
    }

    negate() {
        return new Line(-this.a, -this.b, -this.c);
    }

    parallel_through(point) {
        let c = -(this.a * point[0]) - (this.b * point[1]);
        return new Line(this.a, this.b, c);
    }

    distance_to(point) {
        return this.a * point[0] + this.b * point[1] + this.c;
    }

    draw(canvas, from, to, color) {
        canvas.beginPath();
        canvas.moveTo(from, canvas_size - this.y_at(from));
        canvas.lineTo(to, canvas_size - this.y_at(to));
        canvas.strokeStyle = color;
        canvas.stroke();
    }
}

function line_between(p1, p2) {
    if (p1[0] == p2[0]) {
        return new Line(p1[0], 0, 0);
    } else {
        let slope = (p2[1] - p1[1]) / (p2[0] - p1[0]);
        let offset = -(slope * p1[0]) + p1[1];
        return new Line(slope, -1, offset)
    }
}

class Bezier {
    constructor(p0, p1, p2, p3) {
        this.p0 = p0;
        this.p1 = p1;
        this.p2 = p2;
        this.p3 = p3;
    }

    point_at(t) {
        let ti = 1 - t;
        let x = (Math.pow(ti, 3) * this.p0[0]) + (3 * Math.pow(ti, 2) * t * this.p1[0]) + (3 * ti * Math.pow(t, 2) * this.p2[0]) + (Math.pow(t, 3) * this.p3[0]);
        let y = (Math.pow(ti, 3) * this.p0[1]) + (3 * Math.pow(ti, 2) * t * this.p1[1]) + (3 * ti * Math.pow(t, 2) * this.p2[1]) + (Math.pow(t, 3) * this.p3[1]);
        return [x, y];
    }

    draw(canvas, color) {
        canvas.beginPath();
        canvas.moveTo(this.p0[0], canvas_size - this.p0[1]);
        canvas.bezierCurveTo(
            this.p1[0], canvas_size - this.p1[1],
            this.p2[0], canvas_size - this.p2[1],
            this.p3[0], canvas_size - this.p3[1]);
        canvas.strokeStyle = color;
        canvas.stroke();
    }

    draw_control_points(canvas, color) {
        canvas.beginPath();
        canvas.moveTo(this.p0[0], canvas_size - this.p0[1]);
        canvas.lineTo(this.p1[0], canvas_size - this.p1[1]);
        canvas.lineTo(this.p2[0], canvas_size - this.p2[1]);
        canvas.lineTo(this.p3[0], canvas_size - this.p3[1]);
        canvas.strokeStyle = color;
        canvas.stroke();
    }

    split_at(t) {
        // 1st set
        let x1 = lerp(t, this.p0[0], this.p1[0]);
        let y1 = lerp(t, this.p0[1], this.p1[1]);

        let x2 = lerp(t, this.p1[0], this.p2[0]);
        let y2 = lerp(t, this.p1[1], this.p2[1]);

        let x3 = lerp(t, this.p2[0], this.p3[0]);
        let y3 = lerp(t, this.p2[1], this.p3[1]);

        // 2nd set
        let x4 = lerp(t, x1, x2);
        let y4 = lerp(t, y1, y2);

        let x5 = lerp(t, x2, x3);
        let y5 = lerp(t, y2, y3);

        // 3rd set
        let x6 = lerp(t, x4, x5);
        let y6 = lerp(t, y4, y5);

        let a = new Bezier([this.p0[0], this.p0[1]], [x1, y1], [x4, y4], [x6, y6]);
        let b = new Bezier([x6, y6], [x5, y5], [x3, y3], [this.p3[0], this.p3[1]]);

        return [a, b];
    }

    split_2(low, high) {
        let l = this.split_at(low);
        let h = l[1].split_at((high-low)/(1-low));
        return [l[0], h[0], h[1]];
    }

    /**
     * Returns: [thin, min, max]
     */
    get_fat_line() {
        let thin_line = line_between(this.p0, this.p3);
        let line_1 = thin_line.parallel_through(this.p1);
        let line_2 = thin_line.parallel_through(this.p2);
        let min_c = Math.min(thin_line.c, line_1.c, line_2.c);
        let max_c = Math.max(thin_line.c, line_1.c, line_2.c);
        let min_line = new Line(thin_line.a, thin_line.b, min_c);
        let max_line = new Line(thin_line.a, thin_line.b, max_c);
        return [thin_line, min_line, max_line];
    }

    fat_line_in_range(start, end) {
        let thin_line = line_between(this.point_at(start), this.point_at(end));

        //
    }

    clip_against(line) {
        let e0 = [0, line.distance_to(this.p0)];
        let e1 = [1 / 3, line.distance_to(this.p1)];
        let e2 = [2 / 3, line.distance_to(this.p2)];
        let e3 = [3 / 3, line.distance_to(this.p3)];

        if (e0[1] < 0 && e1[1] < 0 && e2[1] < 0 && e3[1] < 0) {
            return [0, 0];
        }

        var low = 0;
        if (e0[1] < 0) {
            let l1 = line_between(e0, e1);
            let l2 = line_between(e0, e2);
            let l3 = line_between(e0, e3);
            let x1 = l1.x_intercept();
            let x2 = l2.x_intercept();
            let x3 = l3.x_intercept();

            var min = 1000;
            if (x1 > 0 && x1 < min) {
                min = x1;
            }
            if (x2 > 0 && x2 < min) {
                min = x2;
            }
            if (x3 > 0 && x3 < min) {
                min = x3;
            }
            low = min;
        }

        var high = 1;
        if (e3[1] < 0) {
            let l1 = line_between(e0, e3);
            let l2 = line_between(e1, e3);
            let l3 = line_between(e2, e3);
            let x1 = l1.x_intercept();
            let x2 = l2.x_intercept();
            let x3 = l3.x_intercept();
            var max = 0;

            if (x1 < 1 && x1 > max) {
                max = x1;
            }
            if (x2 < 1 && x2 > max) {
                max = x2;
            }
            if (x3 < 1 && x3 > max) {
                max = x3;
            }
            high = max;
        }

        return [low, high];
    }

    find_intersections(other) {
        var a = this;
        var a_clip = [0, 1];
        var b = other;
        var b_clip = [0, 1];
        var iteration = 0;

        while (Math.abs(a_clip[0] - a_clip[1]) > 0.00001) {
            console.log(a_clip, " : ", b_clip);
            console.log(Math.abs(a_clip[0] - a_clip[1]), " : ", Math.abs(b_clip[0] - b_clip[1]))
            if (iteration % 2 == 0) {
                let lines = a.get_fat_line();
                let low = b.clip_against(lines[1].negate());
                let high = b.clip_against(lines[2]);
                let t_start = Math.max(low[0], high[0]);
                let t_end = Math.min(low[1], high[1]);

                b_clip = [t_start, t_end];
                b = b.split_2(t_start, t_end)[1];
            } else {
                let lines = b.get_fat_line();
                let low = a.clip_against(lines[1].negate());
                let high = a.clip_against(lines[2]);
                let t_start = Math.max(low[0], high[0]);
                let t_end = Math.min(low[1], high[1]);

                a_clip = [t_start, t_end];
                a = a.split_2(t_start, t_end)[1];
            }

            iteration++;
        }

        console.log(iteration);
        return [a_clip[0]];
    }
}

let samples = [
    new Sample(
        new Bezier([18, 122], [15, 178], [247, 173], [251, 242]),
        new Bezier([24, 21], [189, 40], [159, 137], [101, 261])
    ),
    // new Sample(
    //     new Bezier([204, 41], [45, 235], [220, 235], [226, 146]),
    //     new Bezier([100, 98], [164, 45], [187, 98], [119, 247])
    // ),
    // new Sample(
    //     new Bezier([18, 122], [15, 178], [247, 173], [251, 242]),
    //     new Bezier([20, 213], [189, 40], [85, 283], [271, 217])
    // )
];

function step1_original(sample, canvas) {
    sample.draw(canvas, true);
}

function step2_thin_line(sample, canvas) {
    sample.draw(canvas, false);

    let thin_line = line_between(sample.curve1.p0, sample.curve1.p3);
    thin_line.draw(canvas, 0, canvas_size, 'black');
}

function step3_thick_line(sample, canvas) {
    sample.curve1.draw_control_points(canvas, 'lightgrey');
    sample.draw(canvas, false);

    let lines = sample.curve1.get_fat_line();
    lines[0].draw(canvas, 0, canvas_size, 'lightgrey');
    lines[1].draw(canvas, 0, canvas_size, 'lightgrey');
    lines[2].draw(canvas, 0, canvas_size, 'lightgrey');
}

function step4_clip(sample, canvas) {
    // sample.draw(canvas, true);

    let lines = sample.curve1.get_fat_line();
    lines[0].draw(canvas, 0, canvas_size, 'lightgrey');
    lines[1].draw(canvas, 0, canvas_size, 'lightgrey');
    lines[2].draw(canvas, 0, canvas_size, 'lightgrey');

    let clip_low = sample.curve2.clip_against(lines[1].negate());
    let clip_high = sample.curve2.clip_against(lines[2]);

    let t_start = Math.max(clip_low[0], clip_high[0]);
    let t_end = Math.min(clip_low[1], clip_high[1]);

    let clipped = sample.curve2.split_2(t_start, t_end);
    sample.curve1.draw(canvas, 'red');
    clipped[0].draw(canvas, 'green');
    clipped[1].draw(canvas, 'blue');
    clipped[2].draw(canvas, 'green');
}

function step5_intersect(sample, canvas) {
    sample.draw(canvas, false);

    let intersections = sample.curve1.find_intersections(sample.curve2);

    intersections.forEach(t => {
        draw_point(canvas, sample.curve1.point_at(t));
    });
}

let steps = [
    [step1_original, "Original"],
    [step2_thin_line, "Thin Line"],
    [step3_thick_line, "Thick Line"],
    [step4_clip, "Clip Bezier"],
    [step5_intersect, "Find Intersections"]
];

function run() {
    console.log(`running ${samples.length} samples`);

    let body = document.body;
    samples.forEach((sample, index) => {
        let row_id = index + 1;

        let row = document.createElement("div");
        row.id = `sample${row_id}`;
        row.className = "sample-row";

        let text = document.createElement("h2");
        text.appendChild(document.createTextNode(`${row_id} Intersections`));
        row.appendChild(text);

        let container = document.createElement("div");
        container.className = "sample-container";

        steps.forEach((step) => {
            let viewport = document.createElement("div");
            viewport.className = "viewport";

            let canvas = document.createElement("canvas");
            canvas.width = canvas_size;
            canvas.height = canvas_size;
            canvas.className = "canvas";
            let context = canvas.getContext("2d");
            context.lineWidth = 2.0;
            step[0](sample, context);

            viewport.appendChild(canvas);

            let text = document.createElement("p");
            text.appendChild(document.createTextNode(step[1]));

            viewport.appendChild(text);
            container.appendChild(viewport);
        })

        row.appendChild(container);
        body.appendChild(row);
    });
}
