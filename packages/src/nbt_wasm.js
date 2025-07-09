let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

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
 * Set panic hook for better debugging
 */
export function main() {
    wasm.main();
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedInt32ArrayMemory0 = null;

function getInt32ArrayMemory0() {
    if (cachedInt32ArrayMemory0 === null || cachedInt32ArrayMemory0.byteLength === 0) {
        cachedInt32ArrayMemory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32ArrayMemory0;
}

function getArrayI32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
 * Detect compression format from bytes
 * @param {Uint8Array} data
 * @returns {string}
 */
export function detectCompression(data) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export_1);
        const len0 = WASM_VECTOR_LEN;
        wasm.detectCompression(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Get version info
 * @returns {string}
 */
export function getVersion() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.getVersion(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Parse SNBT string to NBT tag
 * @param {string} input
 * @returns {JsNbtTag}
 */
export function parseSnbt(input) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        wasm.parseSnbt(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return JsNbtTag.__wrap(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}
/**
 * Format NBT tag to SNBT string
 * @param {JsNbtTag} tag
 * @returns {string}
 */
export function formatSnbt(tag) {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        _assertClass(tag, JsNbtTag);
        wasm.formatSnbt(retptr, tag.__wbg_ptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Format NBT tag to pretty SNBT string with indentation
 * @param {JsNbtTag} tag
 * @returns {string}
 */
export function formatSnbtPretty(tag) {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        _assertClass(tag, JsNbtTag);
        wasm.formatSnbtPretty(retptr, tag.__wbg_ptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
    }
}

const JsNbtFileFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jsnbtfile_free(ptr >>> 0, 1));
/**
 * NBT file wrapper - handles all compression formats
 */
export class JsNbtFile {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JsNbtFile.prototype);
        obj.__wbg_ptr = ptr;
        JsNbtFileFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JsNbtFileFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jsnbtfile_free(ptr, 0);
    }
    /**
     * Read NBT file from bytes
     * @param {Uint8Array} data
     * @returns {JsNbtFile}
     */
    static read(data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export_1);
            const len0 = WASM_VECTOR_LEN;
            wasm.jsnbtfile_read(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JsNbtFile.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Read NBT file with selective field parsing (performance optimization)
     * @param {Uint8Array} data
     * @param {string} fields
     * @returns {JsNbtFile}
     */
    static readFields(data, fields) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export_1);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(fields, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
            const len1 = WASM_VECTOR_LEN;
            wasm.jsnbtfile_readFields(retptr, ptr0, len0, ptr1, len1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JsNbtFile.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get root tag
     * @returns {JsNbtTag}
     */
    get root() {
        const ret = wasm.jsnbtfile_root(this.__wbg_ptr);
        return JsNbtTag.__wrap(ret);
    }
    /**
     * Process multiple paths in one call - avoids WASM round-trips
     * @param {string} paths
     * @returns {any}
     */
    getMultiplePaths(paths) {
        const ptr0 = passStringToWasm0(paths, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.jsnbtfile_getMultiplePaths(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
     * Get file name
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jsnbtfile_name(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get compression format as string
     * @returns {string}
     */
    get compression() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jsnbtfile_compression(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Write NBT file to bytes
     * @returns {Uint8Array}
     */
    write() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jsnbtfile_write(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_0(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Set string value by path - DIRECTLY modifies the internal root
     * @param {string} path
     * @param {string} value
     * @returns {boolean}
     */
    setStringByPath(path, value) {
        const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(value, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.jsnbtfile_setStringByPath(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        return ret !== 0;
    }
    /**
     * Set number value by path - DIRECTLY modifies the internal root
     * @param {string} path
     * @param {number} value
     * @returns {boolean}
     */
    setNumberByPath(path, value) {
        const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.jsnbtfile_setNumberByPath(this.__wbg_ptr, ptr0, len0, value);
        return ret !== 0;
    }
    /**
     * Modify list item by path and index - for compound modifications
     * @param {string} list_path
     * @param {number} index
     * @param {string} key
     * @param {string} value
     * @returns {boolean}
     */
    modifyListItem(list_path, index, key, value) {
        const ptr0 = passStringToWasm0(list_path, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(key, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(value, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.jsnbtfile_modifyListItem(this.__wbg_ptr, ptr0, len0, index, ptr1, len1, ptr2, len2);
        return ret !== 0;
    }
}

const JsNbtRegionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jsnbtregion_free(ptr >>> 0, 1));
/**
 * NBT region file wrapper
 */
export class JsNbtRegion {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JsNbtRegion.prototype);
        obj.__wbg_ptr = ptr;
        JsNbtRegionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JsNbtRegionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jsnbtregion_free(ptr, 0);
    }
    /**
     * Read region from bytes
     * @param {Uint8Array} data
     * @returns {JsNbtRegion}
     */
    static read(data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export_1);
            const len0 = WASM_VECTOR_LEN;
            wasm.jsnbtregion_read(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JsNbtRegion.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create empty region
     * @returns {JsNbtRegion}
     */
    static new() {
        const ret = wasm.jsnbtregion_new();
        return JsNbtRegion.__wrap(ret);
    }
    /**
     * Write region to bytes
     * @returns {Uint8Array}
     */
    write() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jsnbtregion_write(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_0(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get chunk count
     * @returns {number}
     */
    chunkCount() {
        const ret = wasm.jsnbtregion_chunkCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Check if region is empty
     * @returns {boolean}
     */
    isEmpty() {
        const ret = wasm.jsnbtregion_isEmpty(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Get chunk positions as flat array [x1, z1, x2, z2, ...]
     * @returns {Int32Array}
     */
    getChunkPositions() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jsnbtregion_getChunkPositions(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayI32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_0(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get chunk data as NBT tag
     * @param {number} x
     * @param {number} z
     * @returns {JsNbtTag | undefined}
     */
    getChunk(x, z) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jsnbtregion_getChunk(retptr, this.__wbg_ptr, x, z);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0 ? undefined : JsNbtTag.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const JsNbtTagFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jsnbttag_free(ptr >>> 0, 1));
/**
 * NBT tag wrapper for JavaScript - single point of truth
 */
export class JsNbtTag {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JsNbtTag.prototype);
        obj.__wbg_ptr = ptr;
        JsNbtTagFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JsNbtTagFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jsnbttag_free(ptr, 0);
    }
    /**
     * Get tag type ID (matches TypeScript NbtType enum)
     * @returns {number}
     */
    get typeId() {
        const ret = wasm.jsnbttag_typeId(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get as number (0 if not numeric)
     * @returns {number}
     */
    asNumber() {
        const ret = wasm.jsnbttag_asNumber(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get as string (empty if not string)
     * @returns {string}
     */
    asString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jsnbttag_asString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export_0(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get nested value by key (compound only)
     * @param {string} key
     * @returns {JsNbtTag | undefined}
     */
    get(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.jsnbttag_get(this.__wbg_ptr, ptr0, len0);
        return ret === 0 ? undefined : JsNbtTag.__wrap(ret);
    }
    /**
     * Type checking
     * @returns {boolean}
     */
    isNumber() {
        const ret = wasm.jsnbttag_isNumber(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    isString() {
        const ret = wasm.jsnbttag_isString(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    isCompound() {
        const ret = wasm.jsnbttag_isCompound(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    isList() {
        const ret = wasm.jsnbttag_isList(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Get keys for compound tags
     * @returns {string[]}
     */
    keys() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jsnbttag_keys(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_0(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Set a string value by key (compound only)
     * @param {string} key
     * @param {string} value
     * @returns {boolean}
     */
    setString(key, value) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(value, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.jsnbttag_setString(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        return ret !== 0;
    }
    /**
     * Get list length
     * @returns {number}
     */
    listLength() {
        const ret = wasm.jsnbttag_listLength(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get item from list by index
     * @param {number} index
     * @returns {JsNbtTag | undefined}
     */
    getListItem(index) {
        const ret = wasm.jsnbttag_getListItem(this.__wbg_ptr, index);
        return ret === 0 ? undefined : JsNbtTag.__wrap(ret);
    }
    /**
     * Get tag by path (e.g., "Data.Player.Name") - OPTIMIZED RUST PARSING
     * @param {string} path
     * @returns {JsNbtTag | undefined}
     */
    getByPath(path) {
        const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.jsnbttag_getByPath(this.__wbg_ptr, ptr0, len0);
        return ret === 0 ? undefined : JsNbtTag.__wrap(ret);
    }
    /**
     * Set string by path - OPTIMIZED RUST PARSING
     * @param {string} path
     * @param {string} value
     * @returns {boolean}
     */
    setStringByPath(path, value) {
        const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(value, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.jsnbttag_setStringByPath(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        return ret !== 0;
    }
    /**
     * Get string value by path - HIGH PERFORMANCE
     * @param {string} path
     * @returns {string | undefined}
     */
    getStringPath(path) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
            const len0 = WASM_VECTOR_LEN;
            wasm.jsnbttag_getStringPath(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v2;
            if (r0 !== 0) {
                v2 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export_0(r0, r1 * 1, 1);
            }
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get number value by path - HIGH PERFORMANCE
     * @param {string} path
     * @returns {number | undefined}
     */
    getNumberPath(path) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export_1, wasm.__wbindgen_export_2);
            const len0 = WASM_VECTOR_LEN;
            wasm.jsnbttag_getNumberPath(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r2 = getDataViewMemory0().getFloat64(retptr + 8 * 1, true);
            return r0 === 0 ? undefined : r2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_new_5e0be73521bc8c17 = function() {
        const ret = new Map();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_8fc6bf8a5b1071d1 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedInt32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('nbt_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
