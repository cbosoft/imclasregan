var state = { regid: null, lid: null, rid: null, progress: 0, sid: '', time_start: null };

function init(regression_kind) {
    get_image();
    get_regression(regression_kind);
    state.sid = uuidv4();
    state.progress = 0;

    console.log(state.sid);
}

function uuidv4() {
    return ([1e7] + -1e3 + -4e3 + -8e3 + -1e11).replace(/[018]/g, c =>
        (c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> c / 4).toString(16)
    );
}

function send_data(o) {
    return fetch('/site', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(o)
    });
}

function get_one_image() {
    send_data({ command: "GetImage" })
        .then(response => response.json())
        .then(set_image_on_doc);
}

function get_image() {
    var canvas = document.getElementById("image_l");
    var ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    var canvas = document.getElementById("image_r");
    var ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    state.lid = null;
    state.rid = null;
    get_one_image();
    get_one_image();
}

function set_image_on_doc(data) {
    var imagedata = new ImageData(new Uint8ClampedArray(data.data), data.width, data.height);
    var canvas_id = null;
    if (state.lid) {
        if (state.lid != data.iid) {
            state.rid = data.iid;
            canvas_id = "image_r";
        }
        else {
            // left is same as right -> try again
            get_one_image();
            return;
        }
    }
    else {
        state.lid = data.iid;
        canvas_id = "image_l";
    }
    state.start_time = Date.now();
    var aspect_ratio = data.width / data.height;
    var long_length = 300.0;
    var big_width = long_length;
    var big_height = long_length;

    if (aspect_ratio < 1.0) {
        // tall
        big_width = aspect_ratio * big_height;
    }
    else {
        // wide
        big_height = big_width / aspect_ratio;
    }

    createImageBitmap(imagedata, { resizeWidth: big_width, resizeHeight: big_height, resizeQuality: "high" }).then(bitmap => {
        var canvas = document.getElementById(canvas_id);
        var ctx = canvas.getContext('2d');
        canvas.width = big_width;
        canvas.height = big_height;
        ctx.drawImage(bitmap, 0, 0);
    });
}

function get_regression(regression_kind) {
    send_data({ command: "GetRegression", kind: regression_kind })
        .then(response => response.json())
        .then(set_regression_on_doc);
}

function set_regression_on_doc(data) {
    console.log(data);
    var e = document.getElementById("regression_task");
    e.innerHTML = data.in_a_sentence;
    var e = document.getElementById("regression_description");
    e.innerHTML = data.description;
    state.regid = data.rid;
}

function select_left() {
    store_result(state.rid, state.lid);
}

function select_right() {
    store_result(state.lid, state.rid);
}

function store_result(lid, mid) {
    if (state.lid && state.rid) {
        var end_time = Date.now();
        var time_diff = end_time - state.start_time;
        send_data({ command: "StoreRegressionResult", rid: state.regid, lid: lid, mid: mid, sid: state.sid, "tt": time_diff });
        get_image();
        state.progress += 1;
        update_progress_text();
    }
}

function update_progress_text() {
    var e = document.getElementById("thanksetc");

    if (state.progress < 5) {
    }
    else if (state.progress < 10) {
        e.innerHTML = "You've annotated a fair few images, thanks!"
    }
    else {
        e.innerHTML = "You've annotated " + state.progress + " images! Thanks! Your efforts are much appreciated " + "<span class=\"emoji\" > " + String.fromCodePoint(0x2764) + "</span > "
    }
}
