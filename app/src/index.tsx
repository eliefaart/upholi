import * as React from "react";
import * as ReactDOM from "react-dom";
import AppContainer from "./components/AppContainer";
import "./skin/app.scss";

// Render a page in container
const rootElement = document.getElementById("appRoot");
ReactDOM.render(<AppContainer/>, rootElement);

// Can't go async here for some reason.
fetch("dist/test.wasm")
	.then(response => {
		response.arrayBuffer()
			.then(bytes => {

				const importObject: WebAssembly.Imports = {
					imports: {
						test: function(): string {
							return "Hello wasm";
						},
						getEncryptionKey: function(): string {
							return "2ef19a7b730bf8a32533e1cb589c48b6";
						}
					}
				};

				WebAssembly.instantiate(bytes, importObject)
					.then(instance => {
						console.log(instance);
						console.log(instance.instance.exports.add(4, 123));
						console.log(instance.instance.exports.get_number());
						//console.log(instance.instance.exports.test_imported_func());


						//instance.instance.exports.show_string();
					});
			});
	});


//console.log("The answer is: ", instance.exports,greet());