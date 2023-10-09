import init, {
	resize_image,
} from './libs/image_utils/image_utils_wasm.js';
import { openFile, saveFile } from 'react-event-base/FileUtils';
import './index.css';

const store = {};
const images = {};
let table;
let serialID = 0;

function prettifySize(size) {
	let sizeText = size.toString();
	let length = sizeText.length;
	let result = [];
	for(let i = Math.ceil(length / 3); i > 0; i--) {
		result.push(sizeText.substring(length - 3 * (i - 1), length - 3 * i));
	}
	return result.join(',') + ' bytes';
}
function fileExtToMimeType(ext) {
	switch(ext) {
		case '.jpg':
		return 'image/jpg';
	}
}
function createImage(id, file) {
	const img = document.createElement('img');
	Object.assign(img.style, {
		width: '35vmin',
		height: '35vmin',
		objectFit: 'contain',
	});
	img.onload = () => {
		const imgSize = table.element.querySelector('#' + id).querySelector('.f_img_size');
		imgSize.innerText = `Image Size: ${img.width} x ${img.height}`;
		const btnDownload = table.element.querySelector('#' + id).querySelector('button.download');
		btnDownload.addEventListener('click', () => {
			saveFile(file.name, file, true);
		});
	};
	img.src = URL.createObjectURL(file);
	return `<div id="${id}">` +
	'<div class="f_size">' + `File Size: ${prettifySize(file.size)}` + '</div>' +
	'<div class="f_img_size">' + '</div>' +
	img.outerHTML + '</img>' +
	'<button class="download">Download</button>' +
	'</div>';
}
function appendImageItem(id, {original, resized}) {
	table.appendRow([
		createImage('origin_' + id, original),
		createImage('resize_' + id, resized),
	]);
}

Object.assign(window, {
	read_file: (file_path) => {
		// console.log("[ImageGallery] ReadFile: ", file_path);
		return store[file_path];
	},
	write_file: (buffer, file_path) => {
		// console.log("[ImageGallery] WriteFile: ", buffer, file_path);
		const fileExt = file_path.substr(file_path.lastIndexOf('.'));
		const option = {
          type: fileExtToMimeType(fileExt),
        };
        const file = new File([buffer], 'resized' + fileExt, option);
        images[serialID].resized = file;
        appendImageItem(serialID, images[serialID]);
        // saveFile(file.name, file, true);
	},
});

function createButton(args) {
	const btn = document.createElement('button');
	btn.innerText = args.text;
	btn.addEventListener('click', args.click);
	return btn;
}
function createTable() {
	const table = document.createElement('table');
	table.innerHTML = [
	'<thead><tr><th>Original</th><th>Resized</th></tr></thead>',
	'<tbody></tbody>'
	].join('');
	const tbody = table.querySelector('tbody');
	return {
		element: table,
		appendRow: (items) => {
			const tr = document.createElement('tr');
			items.forEach(item => {
				const td = document.createElement('td');
				td.innerHTML = item;
				tr.append(td);
			});
			tbody.append(tr);
		},
	};
}
// ImageUtils.my_rust_fn();
(async () => {
	await init();
	const openBtn = createButton({
		text: "Resize file",
		click: async () => {
			serialID += 1;
			const file = await openFile([".jpg", ".jpeg"]);
			store["file1.jpg"] = new Uint8Array(await file.arrayBuffer());
			images[serialID] = {
				original: file,
			};
			await resize_image("file1.jpg", "file1_out.jpg", 300, 300);
		},
	});
	document.body.append(openBtn);
	table = createTable();
	document.body.append(table.element);
})();
