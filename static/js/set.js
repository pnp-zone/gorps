export class StringSet {

    add(string) {
        this[string] = 1;
        return this;
    }

    update(iterable) {
        if (iterable !== undefined)
            for (const string of iterable)
                this[string] = 1;
        return this;
    }

    remove(string) {
        this[string] = 0;
        return this;
    }

    contains(string) {
        return this[string] === 1;
    }

    toArray() {
        return Object.keys(this);
    }

    [Symbol.iterator]() {
        return Object.keys(this)[Symbol.iterator]();
    }
}
