const rust = import('./pkg/jst');

import "bootstrap";
import 'bootstrap/dist/css/bootstrap.min.css';

import CodeFlask from 'codeflask';
import JSONFormatter from 'json-formatter-js'

// Load a schema fetched via `scripts/mps-download-schemas` and loaded as a raw
// string via webpack.
import schema from "./schemas/glean/baseline/baseline.1.schema.json";

console.log(schema);

const flask = new CodeFlask('#editor', {
    language: 'js',
    lineNumbers: true,
});
flask.updateCode(JSON.stringify(schema, null, 2));

rust.then(jst => {
    let transpiled = jst.convert_bigquery_js(schema);
    console.log(transpiled);
    let formatter = new JSONFormatter(transpiled);
    document.getElementById("output").appendChild(formatter.render());
}).catch(console.error);
