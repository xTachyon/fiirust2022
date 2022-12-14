import init, { base64_encode } from "./pkg/fiirust_wasm.js";

await init();

function file_select(event) {
    // create a file selector
    const reader = new FileReader();
    // call handle_file_load when the user select a file
    reader.onload = handle_file_load;
    reader.readAsArrayBuffer(event.target.files[0]);
}
function handle_file_load(event) {
    // call into the Rust code with the file content
    var output = base64_encode(new Uint8Array(event.target.result));
    var blob = new Blob([output], { type: "text/plain" });

    // JS hack to create a file download with the result
    var link = document.createElement("a");
    link.href = window.URL.createObjectURL(blob);
    link.download = "result.txt";
    link.click();
}

document.getElementById('file_upload').addEventListener("change", file_select);

document.getElementById("submit").addEventListener("click", () => {
    // call into the Rust code with the input from the first textbox,
    // and set it in the second one

    var input = document.getElementById("input_text").value;
    input = new TextEncoder("utf-8").encode(input);
    var output = base64_encode(input);
    document.getElementById("output_text").value = output;
});
