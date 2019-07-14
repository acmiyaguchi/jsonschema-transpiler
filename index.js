const rust = import('./pkg/jst');

import CodeFlask from 'codeflask';
import JSONFormatter from 'json-formatter-js'

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


const flask = new CodeFlask('#editor', {
    language: 'js',
    lineNumbers: true,
    defaultTheme: false
});

rust.then(jst => {
    let transpiled = jst.convert_bigquery_js(schema);
    console.log(transpiled);
    let formatter = new JSONFormatter(transpiled);
    document.body.appendChild(formatter.render());
}).catch(console.error);
