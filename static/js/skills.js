import React, {e} from "./react.js";
import {StringSet} from "./set.js";
import {TableRow} from "./components/table.js";
import {TextInput} from "./components/textinput.js";
import {Select} from "./components/select.js";

export const Difficulty = {
    undefined: "--",
    e: "Easy",
    a: "Average",
    h: "Hard",
    vh: "Very Hard",
};

export const Attributes = {
    undefined: "--",
    st: "ST",
    dx: "DX",
    iq: "IQ",
    ht: "HT",
    per: "Per",
    will: "Will",
};

/**
 * Template of internal skill representation
 */
const SKILL = {
    type: "skill",
    id: String, // uuid
    name: String,
    specialization: String || undefined,
    difficulty: Difficulty,
    attribute: Attributes,
    reference: String,
    // points: number,
    // defaults: Array,
    categories: Array,
}

/**
 * Storage for skills responsable for:
 * - parse objects from source files into desired layout
 * - keep track of categories used
 */
export class SkillList extends Array {
    constructor() {
        super();
        this.categories = new StringSet();
    }

    extendFromSource(obj) {
        if (obj.type !== "skill_list") {
            console.error("Given object is not a skill list");
            return;
        }
        if (obj.version !== 2) {
            console.error("Only skill lists of version 2 are supported");
            return;
        }
        for (let skill of obj.rows) {
            if (skill.type !== "skill") {
                console.warn("Row is not a skill");
                continue;
            }

            const attr_diff = skill.difficulty.split("/");
            if (attr_diff.length === 1) {
                skill.difficulty = attr_diff[0];
            } else if (attr_diff.length === 2) {
                skill.attribute = attr_diff[0];
                skill.difficulty = attr_diff[1];
            } else {
                console.warn(`Unable to parse difficulty: ${difficulty}`);
            }

            this.push(skill);
            this[skill.id] = skill;
            this.categories.update(skill.categories);
        }
    }
}

/**
 * Render a skill into a TableRow
 */
export function SkillComponent({
   id,
   name,
   specialization,
   difficulty,
   attribute,
   categories,
   reference,
   points,
   defaults,
   visible,
}) {
    return e(TableRow, {
        key: id,
        style: visible ? undefined : {display: "none"},
    }, [
        name + (specialization ? ` (${specialization})` : ""),
        Attributes[attribute],
        Difficulty[difficulty],
        categories.map(string => e("span", {}, string)),
        reference,
    ]);
}

/**
 * List all skills in a table
 */
export class SkillTable extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            filter: {
                name: "",
                attribute: "undefined",
                difficulty: "undefined",
                category: "undefined",
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
        if (this.state.filter.attribute !== "undefined")
            filters.push(({attribute}) => attribute === this.state.filter.attribute);
        if (this.state.filter.difficulty !== "undefined")
            filters.push(({difficulty}) => difficulty === this.state.filter.difficulty);
        if (this.state.filter.category !== "undefined")
            filters.push(({categories}) => categories.includes(this.state.filter.category));
        const filter = function (skill) {
            for (const filter of filters)
                if (!filter(skill))
                    return false;
            return true;
        }

        return e("table", {
            className: "skills row-border-hover"
        }, [
            e("thead", {
                className: "stick-top"
            }, [
                e(TableRow, {cell: "th"}, [
                    "Name",
                    "Attribute",
                    "Difficulty",
                    "Categories",
                    "Reference",
                ]),
                e(TableRow, {cell: "th"}, [
                    e(TextInput, {
                        value: this.state.filter.name,
                        setValue: this.createSetter("filter.name"),
                    }),
                    e(Select, {
                        options: Attributes,
                        value: this.state.filter.attribute,
                        setValue: this.createSetter("filter.attribute"),
                    }),
                    e(Select, {
                        options: Difficulty,
                        value: this.state.filter.difficulty,
                        setValue: this.createSetter("filter.difficulty"),
                    }),
                    e(Select, {
                        options: {...Object.fromEntries(this.props.categories.map(c => [c, c])), "undefined": "--"},
                        value: this.state.filter.category,
                        setValue: this.createSetter("filter.category"),
                    }),
                    null,
                ]),
            ]),
            e("tbody", {}, [
                this.props.skills.map(
                    skill => e(SkillComponent, {...skill, visible: filter(skill)})
                )
            ]),
        ]);
    }
}
SkillTable.defaultProps = {
    skills: [],
    categories: [],
};
