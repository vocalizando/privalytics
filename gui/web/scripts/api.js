const RETRIEVE_ENDPOINT = "/api/retrieve";

/**
 * @typedef {Object} AuthData Data used for authentication
 * @property {string} username Username of the user
 * @property {string} token Token of the user
 */

/**
 * @typedef {Object} Entry
 * @property {Object.<string, string | number | boolean>} data Data of the entry
 * @property {Object} metadata
 * @property {number} metadata.date Date of the document, millis
 * @property {string} metadata.duid Unique identifier of the entry
 * @property {string | undefined} metadata.page Page reported by the user
 * @property {string | undefined} metadata.uid Uid of the user
 */

/**
 * Retrieve all entries from the API
 * @param {AuthData} auth_data
 * @return {Entry[]}
 */
async function retrieve_all_entries(auth_data) {
    let response = await fetch(RETRIEVE_ENDPOINT, {
        method: "POST",
        headers: {
            "Accept": "*/*",
            "Content-Type": "application/json",
            "Authorization": generate_authorization_header(auth_data),
        },
        body: JSON.stringify({
            from: 0,
            to: -1
        })
    });

    return await response.json();
}

window.privalytics_api = {
    retrieve_all_entries,
}

/**
 * Generate the contents of the Authorization header from AuthData
 * @param {AuthData} auth_data
 */
function generate_authorization_header(auth_data) {
    return `User ${auth_data.username}:${auth_data.token}`
}
