import init, {resize_image} from './libs/image_utils/image_utils_wasm.js';
import { openFile, saveFile } from 'react-event-base/FileUtils';

const store = {};

function fileExtToMimeType(ext) {
	switch(ext) {
		case '.jpg':
		return 'image/jpg';
	}
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
        saveFile(file.name, file, true);
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
			const file = await openFile(['.jpg', '.jpeg']);
			store['file1.jpg'] = new Uint8Array(await file.arrayBuffer());
			console.log(store['file1.jpg']);
			await resize_image("file1.jpg", "file1_out.jpg", 300, 300);
		},
	});
	document.body.append(openBtn);
})();
