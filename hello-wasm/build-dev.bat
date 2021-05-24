wasm-pack build --target web
copy .\pkg\hello_wasm_bg.wasm ..\app\wwwroot\dist\hello_wasm.wasm
rem cd ..\app\
rem npm install --save ..\hello-wasm\pkg\