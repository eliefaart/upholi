import * as React from "react";
import * as ReactDOM from "react-dom";
import AppContainer from "./components/AppContainer";
import "./skin/app.scss";

import init, {get_struct, get_string, get_array_u32, get_array_u8, aes256_encrypt, aes256_decrypt} from "hello-wasm";

// Render a page in container
const rootElement = document.getElementById("appRoot");
ReactDOM.render(<AppContainer/>, rootElement);

init("dist/hello_wasm.wasm").then(() => {
	const struct = get_struct();
	console.log(struct);
	console.log(struct.id);
	console.log(struct.something);

	console.log(get_array_u32());
	console.log(get_array_u8());
	console.log(get_array_u8().buffer);

	console.log(get_string("abc..def"));

	const bytes = get_array_u8();
	const encrypted = aes256_encrypt(bytes);
	const decrypted = aes256_decrypt(encrypted);
	console.log(bytes, encrypted, decrypted);
});