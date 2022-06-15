function hide_all_top_level_divs() {
    const top_level_divs = Array.from(document.getElementsByClassName("internal.top-level"));

    for (const div of top_level_divs) {
        div.hidden = true;
    }
}

/**
 * Hides all divs and shows the div with the given id
 * @param {string} id
 */
function show_div(id) {
    hide_all_top_level_divs();
    document.getElementById(id).hidden = false;
}

/**
 * Object to raw HTML
 * @param {Object} data
 * @return {string}
 */
function object_to_html(data) {
    let return_value = "";
    for (const key of Object.keys(data)) {
        return_value += `${encodeURI(key)} &rightarrow; ${encodeURI(data[key])}<br>`
    }

    return return_value;
}

/**
 * Create HTML representation of entry
 * @param {AuthData} auth_data
 * @param {Entry[]} entries
 * @return {string}
 */
function show_entries(auth_data, entries) {
    let return_value = '<ul class="list-group">'

    for (const entry of entries.reverse()) {
        const date = new Date(entry.metadata.date);
        return_value += `<li class="list-group-item"><div>
            <b>Metadata</b><br>
            Date: ${date.getDay()}/${date.getMonth()}/${date.getFullYear()}<br>
            DUID: ${entry.metadata.duid}<br>
            ${entry.metadata.page ? `Page: ${encodeURI(entry.metadata.page)}<br>` : ""}
            ${entry.metadata.uid ? `UID: ${entry.metadata.uid}<br>` : ""}
            <b>Data</b><br>
            ${object_to_html(entry.data)}<br>
            <button type="button" class="btn btn-danger" onclick="window.privalytics_api.delete_entry(JSON.parse(decodeURIComponent('${encodeURIComponent(JSON.stringify(auth_data))}')), '${entry.metadata.duid}'); window.loader.reload(JSON.parse(decodeURIComponent('${encodeURIComponent(JSON.stringify(auth_data))}')))">Delete</button>
        </div></li>`
    }

    return_value += "</ul>"
    return return_value;
}

window.utils = {
    hide_all_top_level_divs,
    show_div,
    show_entries,
}
