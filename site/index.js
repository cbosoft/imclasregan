
var current_image = {};
var progress = 0;
var session_id = '';

function uuidv4() {
    return ([1e7] + -1e3 + -4e3 + -8e3 + -1e11).replace(/[018]/g, c =>
        (c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> c / 4).toString(16)
    );
}

function init() {
    get_image();
    get_classes();
    session_id = uuidv4();
    progress = 0;

    console.log(session_id);
}

function get_image() {
    fetch('', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ "command": "get_image" })
    })
        .then(response => response.json())
        .then(set_image_on_doc);
}

function set_image_on_doc(data) {
    var imagedata = new ImageData(new Uint8ClampedArray(data.data), data.width, data.height);
    current_image = { iid: data.iid };
    var aspect_ratio = data.width / data.height;
    var big_height = 300.0;
    var big_width = big_height * aspect_ratio;
    createImageBitmap(imagedata, { resizeWidth: big_width, resizeHeight: big_height, resizeQuality: "high" }).then(bitmap => {
        var canvas = document.getElementById("image");
        var ctx = canvas.getContext('2d');
        canvas.width = bitmap.width;
        canvas.height = bitmap.height;
        ctx.drawImage(bitmap, 0, 0);
    });
}

function get_classes() {
    fetch('', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ "command": "get_classes" })
    })
        .then(response => response.json())
        .then(set_classes_on_doc);
}

function set_classes_on_doc(data) {
    buttons = ""
    for (classdata of data.classes) {
        buttons += "<a class=\"button\" href=\"#\" onclick=\"store_result(" + classdata.cid + ")\">" + classdata.name + "</a><br />";
    }
    var e = document.getElementById("classes");
    e.innerHTML = buttons;
}

function store_result(cid) {
    fetch('', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ "command": "store_result", "cid": cid, "iid": current_image.iid, "sid": session_id })
    });
    get_image();
    progress += 1;
    update_progress_text();
}

function update_progress_text() {
    console.log(progress);
}