import React, {e} from "./react.js";
import ReactDOM from "./react-dom.js";
import {SkillList, SkillTable} from "./skills.js";
import {StringSet} from "./set.js";
import {AdvantageList, AdvantageTable} from "./advantages.js";

class SourceStorage extends StringSet {
    constructor() {
        super();
        //this.update(document.GURPS.sources);
        this.load();
    }

    add(string) {
        super.add(string);
        this.save()
    }
    load() {
        this.update(JSON.parse(localStorage.getItem("sources") || "[]"));
    }
    save() {
        localStorage.setItem("sources", JSON.stringify(this.toArray()));
    }
}

class Main extends React.Component {
    constructor(props) {
        super(props);
        this.sources = new SourceStorage();
        this.skills = new SkillList();
        this.advantages = new AdvantageList();
        for (const source of this.sources) {
            this.loadSource(source);
        }
    }

    loadSource(url) {
        fetch(url).then(response => response.json().then(data => {
            switch (data.type) {
                case "skill_list":
                    this.skills.extendFromSource(data);
                    this.setState({});
                    break;
                case "advantage_list":
                    this.advantages.extendFromSource(data);
                    this.setState({});
                    break;
                default:
                    console.error(`Unknown source type ${data.type}`);
                    break;
            }
            this.sources.add(url);
        }));
    }

    render() {
        return e("div", {
            className: "flex-horizontal",
        }, [
            e(SkillTable, {
                skills: this.skills,
                categories: this.skills.categories.toArray(),
            }),
            /*e(AdvantageTable, {
                advantages: this.advantages,
            }),*/
            e("div", {
                className: "flex-vertical",
            }, [
                e("form", {
                    className: "stick-top",
                    onSubmit: event => {
                        event.preventDefault();
                        this.loadSource(event.target[0].value);
                        event.target[0].value = "";
                    }
                }, [
                    e("h3", {}, "Add source:"),
                    e("input"),
                ]),
            ]),
        ]);
    }
}

ReactDOM.render(
    e(Main),
    document.body,
);
