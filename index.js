const rust = import('./pkg/jst');

import "jstree";
import "jstree/dist/themes/default/style.css";
import $ from 'jquery';
window.jQuery = $;
window.$ = $;

let schema = {
    "type": "object",
    "properties": {
        "id": {
            "type": "string"
        },
        "payload": {
            "type": "object",
            "properties": {
                "timestamp": {
                    "type": "integer"
                },
                "value": {
                    "type": "integer"
                }
            }
        }
    }
}

rust.then(jst => {
    let transpiled = jst.convert_bigquery_js(schema);
    console.log(transpiled);
    $("#display").on("changed.jstree", (e, data) => {
        var i, j, r = [];
        for(i = 0, j = data.selected.length; i < j; i++) {
          r.push(data.instance.get_node(data.selected[i]).text);
        }
        $('#result').html('Selected: ' + r.join(', '));
    }).on("select_node.jstree", (e, data) => {
       data.instance.toggle_node(data.node);
    })
    .jstree({
        "core": {
            "animation" : 0,
            "plugins": ["wholerow"],
            "data": [
                { "id" : "ajson1", "parent" : "#", "text" : "Simple root node" },
                { "id" : "ajson2", "parent" : "#", "text" : "Root node 2" },
                { "id" : "ajson3", "parent" : "ajson2", "text" : "Child 1" },
                { "id" : "ajson4", "parent" : "ajson2", "text" : "Child 2" },
            ]
        }
    });
}).catch(console.error);
