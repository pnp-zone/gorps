import React, {e} from "./react.js";
import {TableRow} from "./components/table.js";
import {TextInput} from "./components/textinput.js";
import {Select} from "./components/select.js";

/**
 * Template of internal advantage representation
 */
const ADVANTAGES = {
    type: "advantage",
    "id": String, // uuid
    "name": String,
    "mental": Boolean || undefined,
    "physical": Boolean || undefined,
    "social": Boolean || undefined,
    "exotic": Boolean || undefined,
    "supernatural": Boolean || undefined,
    "base_points": Number || undefined,
    "points_per_level": Number || undefined,
    "modifiers": Array,
    "features": Array,
    "reference": String,
    "cr": Number || undefined,
    "calc": {
        "points": Number,
    } || undefined,
    "categories": Array,

    "tags": Array, // composition of above booleans
};

const TAGS = {
    exotic: "1",
    mental: "2",
    physical: "3",
    social: "4",
    supernatural: "5",
    pistol: "6",
};

export class AdvantageList extends Array {
    constructor() {
        super();
    }

    extendFromSource(obj) {
        if (obj.type !== "advantage_list") {
            console.error("Given object is not an advantage list");
            return;
        }
        if (obj.version !== 2) {
            console.error("Only skill lists of version 2 are supported");
            return;
        }
        for (let advantage of obj.rows) {
            if (advantage.type !== "advantage") {
                console.warn("Row is not a advantage");
                continue;
            }

            advantage.tags = [];
            for (const [tag, id] of Object.entries(TAGS))
                if (advantage[tag])
                    advantage.tags.push(id);

            this.push(advantage);
            this[advantage.id] = advantage;
        }
    }
}

export function AdvantageComponent({
    id,
    name,
    tags,
    base_points,
    points_per_level,
    reference,
    cr,
    visible,
}) {
    const selfControl = cr === undefined ? "" : "*";

    let cost;
    if (points_per_level !== undefined)
        cost = `${points_per_level}${selfControl} / lvl`;
    else if (base_points !== undefined)
        cost = `${base_points}${selfControl}`;
    else
        cost = `var${selfControl}`;

    return e(TableRow, {
        key: id,
        style: visible ? undefined : {display: "none"},
    }, [
        name,
        tags.join(" "),
        cost,
        reference,
    ]);
}

export class AdvantageTable extends React.Component {
    constructor(props) {
        super();
        this.state = {
            filter: {
                name: "",
                tag: "0",
            }
        };
    }

    createSetter(path) {
        path = path.split(".");
        const last = path.pop();
        return function (value) {
            this.setState(function (state) {
                const result = {...state};
                let oldState = state;
                let newState = result;
                for (const key of path) {
                    newState[key] = {...oldState[key]};
                    oldState = oldState[key];
                    newState = newState[key];
                }
                newState[last] = value;
                return result;
            });
        }.bind(this);
    }

    render() {
        const filters = [];
        if (this.state.filter.name.length > 0) {
            filters.push(({name}) => name.includes(this.state.filter.name));
        }
        if (this.state.filter.tag !== "0")
            filters.push(({tags}) => tags.includes(this.state.filter.tag));
        const filter = function (skill) {
            for (const filter of filters)
                if (!filter(skill))
                    return false;
            return true;
        }

        return e("table", {
            className: "advantages row-border-hover",
        }, [
            e("thead", {
                className: "stick-top"
            }, [
                e(TableRow, {cell: "th"}, [
                    "Name",
                    "Tags",
                    "Points",
                    "Reference",
                ]),
                e(TableRow, {cell: "th"}, [
                    e(TextInput, {
                        value: this.state.filter.name,
                        setValue: this.createSetter("filter.name"),
                    }),
                    e(Select, {
                        value: this.state.filter.tag,
                        setValue: this.createSetter("filter.tag"),
                        options: ["--", "Exotic", "Mental", "Physical", "Social", "Supernatural"],
                    }),
                    null,
                    null,
                ]),
            ]),
            e("tbody", {}, [
                this.props.advantages.map(
                    adv => e(AdvantageComponent, {...adv, visible: filter(adv)})
                ),
            ]),
        ])
    }
}