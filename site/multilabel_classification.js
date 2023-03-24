var state = { iid: null, progress: 0, sid: '', time_start: null, labels: [] };

function init() {
    get_image();
    get_classes();
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

function get_image() {
    var canvas = document.getElementById("image");
    var ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    state.iid = null;
    send_data({ command: "GetImage" })
        .then(response => response.json())
        .then(set_image_on_doc);
}

function set_image_on_doc(data) {
    var imagedata = new ImageData(new Uint8ClampedArray(data.data), data.width, data.height);
    state.iid = data.iid;
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
        var canvas = document.getElementById("image");
        var ctx = canvas.getContext('2d');
        canvas.width = bitmap.width;
        canvas.height = bitmap.height;
        ctx.drawImage(bitmap, 0, 0);
    });

    for (cid in state.classes) {
        var e = document.getElementById("button_" + cid);
        if (e) {
            e.classList.remove("selected");
        }
        state.classes[cid] = false;
    }
}

function get_classes() {
    send_data({ command: "GetClassifications" })
        .then(response => response.json())
        .then(set_classes_on_doc);
}

function set_classes_on_doc(data) {
    var buttons = ""
    var n = 0;
    for (classdata of data) {
        buttons += "<a id=\"button_" + classdata.cid + "\" class=\"button\" href=\"#\" onclick=\"toggle_label(" + classdata.cid + ");\"><b>" + classdata.name + "</b><br/>" + classdata.description + "</a>";
        n = Math.max(n, classdata.cid);
    }
    state.classes = new Array(n).fill(false);
    var e = document.getElementById("classes");
    e.innerHTML = buttons;
}

function toggle_label(cid) {
    state.classes[cid] = !state.classes[cid];
    var e = document.getElementById("button_" + cid);
    if (state.classes[cid]) {
        e.classList.add("selected");
    }
    else {
        e.classList.remove("selected");
    }
}

function store_result() {
    if (state.iid) {
        var end_time = Date.now();
        var time_diff = end_time - state.start_time;
        console.log(state.classes);
        for (cid in state.classes) {
            if (state.classes[cid]) {
                console.log("image " + state.iid + " marked as " + cid);
                send_data({ command: "StoreMultilabelClassificationResult", cid: Number(cid), iid: Number(state.iid), sid: state.sid, "tt": time_diff });
            }
        }
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
