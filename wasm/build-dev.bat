wasm-pack build --target web
copy .\pkg\wasm_bg.wasm ..\app\wwwroot\dist\wasm.wasm
rem cd ..\app\
rem npm install --save ..\wasm\pkg\