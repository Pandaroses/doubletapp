let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}
/**
 * @param {number} x
 * @returns {number}
 */
export function sigmoid(x) {
    const ret = wasm.sigmoid(x);
    return ret;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

const SplitMixFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_splitmix_free(ptr >>> 0, 1));

export class SplitMix {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SplitMixFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_splitmix_free(ptr, 0);
    }
    /**
     * @param {bigint | undefined} [seed]
     */
    constructor(seed) {
        const ret = wasm.splitmix_new(!isLikeNone(seed), isLikeNone(seed) ? BigInt(0) : seed);
        this.__wbg_ptr = ret >>> 0;
        SplitMixFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {bigint}
     */
    next() {
        const ret = wasm.splitmix_next(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
}

const Xoshiro256plusFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_xoshiro256plus_free(ptr >>> 0, 1));

export class Xoshiro256plus {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Xoshiro256plusFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_xoshiro256plus_free(ptr, 0);
    }
    /**
     * @param {bigint | undefined} [seed]
     */
    constructor(seed) {
        const ret = wasm.xoshiro256plus_new(!isLikeNone(seed), isLikeNone(seed) ? BigInt(0) : seed);
        this.__wbg_ptr = ret >>> 0;
        Xoshiro256plusFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    next() {
        const ret = wasm.xoshiro256plus_next(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    get_seed() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.xoshiro256plus_get_seed(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

