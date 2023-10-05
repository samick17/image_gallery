import init, {
	resize_image,
} from './libs/image_utils/image_utils_wasm.js';
import { openFile, saveFile } from 'react-event-base/FileUtils';
import './index.css';

const store = {};
const images = {};
let table;
let serialID = 0;

function fileExtToMimeType(ext) {
	switch(ext) {
		case '.jpg':
		return 'image/jpg';
	}
}
function createImage(image) {
	const img = document.createElement('img');
	Object.assign(img.style, {
		width: '15vmin',
		height: '15vmin',
		objectFit: 'contain',
	})
	img.src = URL.createObjectURL(image);
	// (parent || document.body).append(img);
	return img.outerHTML + '</img>';
}
function appendImageItem({original, resized}) {
	// const parent = document.createElement('div');
	table.appendRow([
		createImage(original),
		createImage(resized),
	]);
	// document.body.append(parent);
}

Object.assign(window, {
	read_file: (file_path) => {
		console.log("[ImageGallery] ReadFile: ", file_path);
		return store[file_path];
	},
	write_file: (buffer, file_path) => {
		console.log("[ImageGallery] WriteFile: ", buffer, file_path);
		const fileExt = file_path.substr(file_path.lastIndexOf('.'));
		const option = {
          type: fileExtToMimeType(fileExt),
        };
        const file = new File([buffer], 'resized' + fileExt, option);
        images[serialID].resized = file;
        // saveFile(file.name, file, true);
        appendImageItem(images[serialID]);
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
