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

function init() {
    send_data({ command: 'GetDatasetSummary' })
        .then(response => response.json())
        .then(set_summary_on_doc);
}

function set_summary_on_doc(o) {
    document.getElementById("singlelabel_results").style.display = o.classified_image_count ? '' : 'none';
    document.getElementById("multilabel_results").style.display = o.multilabelclassified_image_count ? '' : 'none';
    document.getElementById("total_images").innerHTML = "" + o.image_count;
    document.getElementById("classified_images").innerHTML = "" + o.classified_image_count;
    document.getElementById("total_annotations").innerHTML = "" + o.classified_annotations_count;
    document.getElementById("multilabel_classified_images").innerHTML = "" + o.multilabelclassified_image_count;
    document.getElementById("total_multilabel_annotations").innerHTML = "" + o.multilabelclassified_annotations_count;

}

init();