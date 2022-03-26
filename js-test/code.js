
class Sample {
    constructor(curve1, curve2) {
        this.curve1 = curve1;
        this.curve2 = curve2;
    }
}

class Bezier {
    constructor(p0, p1, p2, p3) {
        this.p0 = p0;
        this.p1 = p1;
        this.p2 = p2;
        this.p3 = p3;
    }

    draw(canvas, color) {
        canvas.beginPath();
        canvas.moveTo(this.p0[0], this.p0[1]);
        canvas.bezierCurveTo(this.p1[0], this.p1[1], this.p2[0], this.p2[1], this.p3[0], this.p3[1]);
        canvas.strokeStyle = color;
        canvas.stroke();
    }

    draw_control_points(canvas, color) {
        canvas.beginPath();
        canvas.moveTo(this.p0[0], this.p0[1]);
        canvas.lineTo(this.p1[0], this.p1[1]);
        canvas.lineTo(this.p2[0], this.p2[1]);
        canvas.lineTo(this.p3[0], this.p3[1]);
        canvas.strokeStyle = color;
        canvas.stroke();
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
        return (this.a * x + this.c) / -this.b;
    }

    slope() {
        return this.a
    }

    parallel_through(point) {
        let c = -(this.a * point[0]) - (this.b * point[1]);
        return new Line(this.a, this.b, c);
    }

    distance_to(point) {
        return Math.abs(this.a * point[0] + this.b * point[1] + this.c);
    }

    draw(canvas, from, to, color) {
        canvas.beginPath();
        canvas.moveTo(from, this.y_at(from));
        canvas.lineTo(to, this.y_at(to));
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

let samples = [
    new Sample(
        new Bezier([18, 122], [15, 178], [247, 173], [251, 242]),
        new Bezier([24, 21], [189, 40], [159, 137], [101, 261])
    ),
    new Sample(
        new Bezier([204, 41], [45, 235], [220, 235], [226, 146]),
        new Bezier([100, 98], [164, 45], [187, 98], [119, 247])
    ),
    new Sample(
        new Bezier([18, 122], [15, 178], [247, 173], [251, 242]),
        new Bezier([20, 213], [189, 40], [85, 283], [271, 217])
    )
];

let limit_x = 300;

function step1_original(sample, canvas) {
    sample.curve1.draw(canvas, 'red');
    sample.curve1.draw_control_points(canvas, 'lightgrey');

    sample.curve2.draw(canvas, 'blue');
    sample.curve2.draw_control_points(canvas, 'lightgrey');
}

function step2_thin_line(sample, canvas) {
    sample.curve1.draw(canvas, 'red');
    sample.curve2.draw(canvas, 'blue');

    let thin_line = line_between(sample.curve1.p0, sample.curve1.p3);
    thin_line.draw(canvas, 0, limit_x, 'black');
}

function step3_thick_line(sample, canvas) {
    sample.curve1.draw(canvas, 'red');
    sample.curve2.draw(canvas, 'blue');

    sample.curve1.draw_control_points(canvas, 'lightgrey');

    let thin_line = line_between(sample.curve1.p0, sample.curve1.p3);
    thin_line.draw(canvas, 0, limit_x, 'lightgrey')

    let line_1 = thin_line.parallel_through(sample.curve1.p1);
    line_1.draw(canvas, 1, limit_x, 'grey');

    let line_2 = thin_line.parallel_through(sample.curve1.p2);
    line_2.draw(canvas, 1, limit_x, 'grey');

    let min_c = Math.min(thin_line.c, line_1.c, line_2.c);
    let max_c = Math.max(thin_line.c, line_1.c, line_2.c);

    let min_line = new Line(thin_line.a, thin_line.b, min_c);
    min_line.draw(canvas, 0, limit_x, 'black');
    let max_line = new Line(thin_line.a, thin_line.b, max_c);
    max_line.draw(canvas, 0, limit_x, 'black');
}

let steps = [
    [step1_original, "Original"],
    [step2_thin_line, "Thin Line"],
    [step3_thick_line, "Thick Line"],
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
            canvas.width = 200;
            canvas.height = 200;
            canvas.className = "canvas";
            let context = canvas.getContext("2d");
            context.scale(0.66666, 0.66666);
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
