
export const getApiURL = () => {
    const apiURL = new URL(window.location.href);
    if (apiURL.href.endsWith("index.html")) {
        apiURL.href = apiURL.href.replace("index.html", "");
    }
    apiURL.port = 8282; // if env.DEV => set api port to 8282
    console.log(apiURL);
    return apiURL;
}