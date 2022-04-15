import {e} from "../react.js";

export function TableRow({children, cell, ...props}) {
    return e("tr", props, children.map(elem => e(cell, {}, elem)));
}
TableRow.defaultProps = {cell: "td"}
