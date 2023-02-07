let exportBtn = document.getElementById("export");
exportBtn.onclick = exportBookmarks;

function exportBookmarks() {
    const bookmarks = []
    chrome.bookmarks.getTree(function (bookmarkArray) {
        buildList(bookmarkArray, bookmarks)
        const json = JSON.stringify(bookmarks)
        let eleLink = document.createElement('a');
        eleLink.download = 'bookmarks.json';
        eleLink.style.display = 'none'
        let blob = new Blob([json]);
        eleLink.href = URL.createObjectURL(blob)
        document.body.appendChild(eleLink)
        eleLink.click()
        document.body.removeChild(eleLink)
    });
}

// 处理书签
function buildList(val, data) {
    for (let i = 0; i < val.length; i++) {
        if (val[i].url !== undefined) {
            data.push({title: val[i].title, url: val[i].url})
        }
        if (val[i].children !== undefined) {
            buildList(val[i].children, data)
        }
    }
}
