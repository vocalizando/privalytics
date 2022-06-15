const api = window.privalytics_api;
const utils = window.utils;

async function run(auth_data) {
    const user = auth_data || retrieve_user_data();

    if (user == null) {
        utils.show_div("error.invalid-credentials")
        return;
    }

    try {
        const entries = await api.retrieve_all_entries(user);
        const html = utils.show_entries(user, entries);

        const div = document.getElementById("result.inner.list");
        div.innerHTML = html;

        show_div("result.list")
    } catch(e) {
        show_div("error.invalid-credentials")
    }

}

/**
 * @return {AuthData | undefined}
 */
function retrieve_user_data() {
    const username = prompt("Username");
    const token = prompt("Token");

    if (username == null || token == null) {
        return;
    }

    return {
        username,
        token,
    }
}

window.loader = {
    reload: run,
}

run();
