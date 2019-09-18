import * as wasm from "../pkg";

// wasm.greet("WOA");

document.querySelector('#to_json').addEventListener('click', convertToJson)
document.querySelector('#to_toml').addEventListener('click', convertToToml)

function convertToToml(){
    convert(wasm.convert_json_to_toml)
}

function convertToJson(){
    convert(wasm.convert_toml_to_json)
}


function convert(fun){
    try{
        document.getElementById("output").value = fun(document.getElementById("input").value);
    }catch(e){
        document.getElementById("output").value = e.toString()
    }
}

