import * as wasm from "../pkg";

// wasm.greet("WOA");

document.querySelector('#to_json').addEventListener('click', convertToJson)
document.querySelector('#to_toml').addEventListener('click', convertToToml)

function convertToToml(){
    try{
        document.getElementById("output").value = wasm.convert_json_to_toml(document.getElementById("input").value);
    }catch(e){
        document.getElementById("output").value = e.toString()
    }
}

function convertToJson(){
    try{
        document.getElementById("output").value = wasm.convert_toml_to_json(document.getElementById("input").value);
    }catch(e){
        document.getElementById("output").value = e.toString()
    }
}

