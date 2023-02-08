let address = 'localhost:8081/backend'

document.getElementById('search').addEventListener('input', () => {
    let search = document.getElementById('search').value
    let content = document.getElementById('content');
    content.innerHTML = '';
    if (!search) {
        return;
    }
    fetch(`http://${address}/search`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            params: search
        })
    })
        .then(res => res.json())
        .then(dataArray => {
            let html = '';
            dataArray.forEach(data => {
                html = '<div class="card">\n' +
                    '<div class="title">\n' +
                    `${data.title}\n` +
                    '</div>\n' +
                    '<div class="url">\n' +
                    `${data.url}\n` +
                    '</div>\n' +
                    '<div class="tags">\n' +
                    `${data.tags}\n` +
                    '</div>\n' +
                    '</div>\n'
            });
            content.innerHTML = html;
        })
})
