import init, {
	resize_image,
} from './libs/image_utils/image_utils_wasm.js';
import { openFile, saveFile } from 'react-event-base/FileUtils';

const store = {};
const images = {};
let serialID = 0;

function fileExtToMimeType(ext) {
	switch(ext) {
		case '.jpg':
		return 'image/jpg';
	}
}
function appendImage(image, parent) {
	const img = document.createElement('img');
	Object.assign(img.style, {
		width: '15vmin',
		height: '15vmin',
		objectFit: 'contain',
	})
	img.src = URL.createObjectURL(image);
	(parent || document.body).append(img);
}
function appendImageItem({original, resized}) {
	const parent = document.createElement('div');
	appendImage(original, parent);
	appendImage(resized, parent);
	document.body.append(parent);
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

// ImageUtils.my_rust_fn();
(async () => {
	await init();
	const openBtn = createButton({
		text: 'Resize file',
		click: async () => {
			serialID += 1;
			const file = await openFile(['.jpg', '.jpeg']);
			store['file1.jpg'] = new Uint8Array(await file.arrayBuffer());
			console.log(store['file1.jpg']);
			images[serialID] = {
				original: file,
			};
			await resize_image("file1.jpg", "file1_out.jpg", 300, 300);
		},
	});
	document.body.append(openBtn);
})();
