(() => {
  var nn = "computed",
    Ie = {
      type: 1,
      name: nn,
      keyReq: 1,
      valReq: 1,
      onLoad: ({ key: t, signals: e, genRX: n }) => {
        let r = n();
        e.setComputed(t, r);
      },
    };
  var O = (t) => t.trim() === "true",
    $ = (t) =>
      t.replace(
        /[A-Z]+(?![a-z])|[A-Z]/g,
        (e, n) => (n ? "-" : "") + e.toLowerCase()
      ),
    Le = (t) =>
      t
        .replace(/(?:^\w|[A-Z]|\b\w)/g, function (e, n) {
          return n == 0 ? e.toLowerCase() : e.toUpperCase();
        })
        .replace(/\s+/g, ""),
    ne = (t) => new Function(`return Object.assign({}, ${t})`)();
  var ke = {
    type: 1,
    name: "signals",
    valReq: 1,
    removeOnLoad: !0,
    onLoad: (t) => {
      let { key: e, genRX: n, signals: r } = t;
      if (e != "") r.setValue(e, n()());
      else {
        let i = ne(t.value);
        (t.value = JSON.stringify(i)), r.merge(n()());
      }
    },
  };
  var Ce = {
    type: 1,
    name: "star",
    keyReq: 2,
    valReq: 2,
    onLoad: () => {
      alert("YOU ARE PROBABLY OVERCOMPLICATING IT");
    },
  };
  var De = {
    name: "signalValue",
    type: 0,
    fn: (t) => {
      let e = /(?<path>[\w0-9.]*)((\.value))/gm;
      return t.replaceAll(e, "ctx.signals.signal('$1').value");
    },
  };
  var L = "datastar";
  var Ve = "Datastar-Request",
    Oe = "0.21.3";
  var Fe = "type module";
  var k = {
      Morph: "morph",
      Inner: "inner",
      Outer: "outer",
      Prepend: "prepend",
      Append: "append",
      Before: "before",
      After: "after",
      UpsertAttributes: "upsertAttributes",
    },
    He = k.Morph,
    I = {
      MergeFragments: "datastar-merge-fragments",
      MergeSignals: "datastar-merge-signals",
      RemoveFragments: "datastar-remove-fragments",
      RemoveSignals: "datastar-remove-signals",
      ExecuteScript: "datastar-execute-script",
    };
  function qe(t) {
    if (t.id) return t.id;
    let e = 0,
      n = (i) => ((e = (e << 5) - e + i), e & e),
      r = (i) => i.split("").forEach((s) => n(s.charCodeAt(0)));
    for (; t.parentNode; ) {
      if (t.id) {
        r(`${t.id}`);
        break;
      } else if (t === t.ownerDocument.documentElement) r(t.tagName);
      else {
        for (
          let i = 1, s = t;
          s.previousElementSibling;
          s = s.previousElementSibling, i++
        )
          n(i);
        t = t.parentNode;
      }
      t = t.parentNode;
    }
    return L + e;
  }
  function We(t, e) {
    let n = new MutationObserver((r) => {
      for (let i of r)
        for (let s of i.removedNodes)
          if (s === t) {
            n.disconnect(), e();
            return;
          }
    });
    n.observe(t.parentNode, { childList: !0 });
  }
  var rn = "https://data-star.dev/errors";
  var f = (t, e) => {
    let n = new Error();
    (t = t.charAt(0).toUpperCase() + t.slice(1)), (n.name = `error ${t}`);
    let r = `${rn}/${t}?${new URLSearchParams(e)}`;
    return (n.message = `for more info see ${r}`), n;
  };
  var sn = Symbol.for("preact-signals"),
    D = 1,
    B = 2,
    X = 4,
    j = 8,
    re = 16,
    G = 32;
  function ye() {
    ie++;
  }
  function Ee() {
    if (ie > 1) {
      ie--;
      return;
    }
    let t,
      e = !1;
    for (; z !== void 0; ) {
      let n = z;
      for (z = void 0, be++; n !== void 0; ) {
        let r = n._nextBatchedEffect;
        if (
          ((n._nextBatchedEffect = void 0),
          (n._flags &= ~B),
          !(n._flags & j) && $e(n))
        )
          try {
            n._callback();
          } catch (i) {
            e || ((t = i), (e = !0));
          }
        n = r;
      }
    }
    if (((be = 0), ie--, e)) throw f("BatchError, error", { error: t });
  }
  var E;
  var z,
    ie = 0,
    be = 0,
    se = 0;
  function Ue(t) {
    if (E === void 0) return;
    let e = t._node;
    if (e === void 0 || e._target !== E)
      return (
        (e = {
          _version: 0,
          _source: t,
          _prevSource: E._sources,
          _nextSource: void 0,
          _target: E,
          _prevTarget: void 0,
          _nextTarget: void 0,
          _rollbackNode: e,
        }),
        E._sources !== void 0 && (E._sources._nextSource = e),
        (E._sources = e),
        (t._node = e),
        E._flags & G && t._subscribe(e),
        e
      );
    if (e._version === -1)
      return (
        (e._version = 0),
        e._nextSource !== void 0 &&
          ((e._nextSource._prevSource = e._prevSource),
          e._prevSource !== void 0 &&
            (e._prevSource._nextSource = e._nextSource),
          (e._prevSource = E._sources),
          (e._nextSource = void 0),
          (E._sources._nextSource = e),
          (E._sources = e)),
        e
      );
  }
  function R(t) {
    (this._value = t),
      (this._version = 0),
      (this._node = void 0),
      (this._targets = void 0);
  }
  R.prototype.brand = sn;
  R.prototype._refresh = function () {
    return !0;
  };
  R.prototype._subscribe = function (t) {
    this._targets !== t &&
      t._prevTarget === void 0 &&
      ((t._nextTarget = this._targets),
      this._targets !== void 0 && (this._targets._prevTarget = t),
      (this._targets = t));
  };
  R.prototype._unsubscribe = function (t) {
    if (this._targets !== void 0) {
      let e = t._prevTarget,
        n = t._nextTarget;
      e !== void 0 && ((e._nextTarget = n), (t._prevTarget = void 0)),
        n !== void 0 && ((n._prevTarget = e), (t._nextTarget = void 0)),
        t === this._targets && (this._targets = n);
    }
  };
  R.prototype.subscribe = function (t) {
    return oe(() => {
      let e = this.value,
        n = E;
      E = void 0;
      try {
        t(e);
      } finally {
        E = n;
      }
    });
  };
  R.prototype.valueOf = function () {
    return this.value;
  };
  R.prototype.toString = function () {
    return this.value + "";
  };
  R.prototype.toJSON = function () {
    return this.value;
  };
  R.prototype.peek = function () {
    let t = E;
    E = void 0;
    try {
      return this.value;
    } finally {
      E = t;
    }
  };
  Object.defineProperty(R.prototype, "value", {
    get() {
      let t = Ue(this);
      return t !== void 0 && (t._version = this._version), this._value;
    },
    set(t) {
      if (t !== this._value) {
        if (be > 100) throw f("SignalCycleDetected");
        (this._value = t), this._version++, se++, ye();
        try {
          for (let e = this._targets; e !== void 0; e = e._nextTarget)
            e._target._notify();
        } finally {
          Ee();
        }
      }
    },
  });
  function $e(t) {
    for (let e = t._sources; e !== void 0; e = e._nextSource)
      if (
        e._source._version !== e._version ||
        !e._source._refresh() ||
        e._source._version !== e._version
      )
        return !0;
    return !1;
  }
  function Be(t) {
    for (let e = t._sources; e !== void 0; e = e._nextSource) {
      let n = e._source._node;
      if (
        (n !== void 0 && (e._rollbackNode = n),
        (e._source._node = e),
        (e._version = -1),
        e._nextSource === void 0)
      ) {
        t._sources = e;
        break;
      }
    }
  }
  function Ge(t) {
    let e = t._sources,
      n;
    for (; e !== void 0; ) {
      let r = e._prevSource;
      e._version === -1
        ? (e._source._unsubscribe(e),
          r !== void 0 && (r._nextSource = e._nextSource),
          e._nextSource !== void 0 && (e._nextSource._prevSource = r))
        : (n = e),
        (e._source._node = e._rollbackNode),
        e._rollbackNode !== void 0 && (e._rollbackNode = void 0),
        (e = r);
    }
    t._sources = n;
  }
  function q(t) {
    R.call(this, void 0),
      (this._fn = t),
      (this._sources = void 0),
      (this._globalVersion = se - 1),
      (this._flags = X);
  }
  q.prototype = new R();
  q.prototype._refresh = function () {
    if (((this._flags &= ~B), this._flags & D)) return !1;
    if (
      (this._flags & (X | G)) === G ||
      ((this._flags &= ~X), this._globalVersion === se)
    )
      return !0;
    if (
      ((this._globalVersion = se),
      (this._flags |= D),
      this._version > 0 && !$e(this))
    )
      return (this._flags &= ~D), !0;
    let t = E;
    try {
      Be(this), (E = this);
      let e = this._fn();
      (this._flags & re || this._value !== e || this._version === 0) &&
        ((this._value = e), (this._flags &= ~re), this._version++);
    } catch (e) {
      (this._value = e), (this._flags |= re), this._version++;
    }
    return (E = t), Ge(this), (this._flags &= ~D), !0;
  };
  q.prototype._subscribe = function (t) {
    if (this._targets === void 0) {
      this._flags |= X | G;
      for (let e = this._sources; e !== void 0; e = e._nextSource)
        e._source._subscribe(e);
    }
    R.prototype._subscribe.call(this, t);
  };
  q.prototype._unsubscribe = function (t) {
    if (
      this._targets !== void 0 &&
      (R.prototype._unsubscribe.call(this, t), this._targets === void 0)
    ) {
      this._flags &= ~G;
      for (let e = this._sources; e !== void 0; e = e._nextSource)
        e._source._unsubscribe(e);
    }
  };
  q.prototype._notify = function () {
    if (!(this._flags & B)) {
      this._flags |= X | B;
      for (let t = this._targets; t !== void 0; t = t._nextTarget)
        t._target._notify();
    }
  };
  Object.defineProperty(q.prototype, "value", {
    get() {
      if (this._flags & D) throw f("SignalCycleDetected");
      let t = Ue(this);
      if (
        (this._refresh(),
        t !== void 0 && (t._version = this._version),
        this._flags & re)
      )
        throw f("GetComputedError", { value: this._value });
      return this._value;
    },
  });
  function je(t) {
    return new q(t);
  }
  function Ke(t) {
    let e = t._cleanup;
    if (((t._cleanup = void 0), typeof e == "function")) {
      ye();
      let n = E;
      E = void 0;
      try {
        e();
      } catch (r) {
        throw (
          ((t._flags &= ~D),
          (t._flags |= j),
          Se(t),
          f("CleanupEffectError", { error: r }))
        );
      } finally {
        (E = n), Ee();
      }
    }
  }
  function Se(t) {
    for (let e = t._sources; e !== void 0; e = e._nextSource)
      e._source._unsubscribe(e);
    (t._fn = void 0), (t._sources = void 0), Ke(t);
  }
  function on(t) {
    if (E !== this) throw f("EndEffectError");
    Ge(this), (E = t), (this._flags &= ~D), this._flags & j && Se(this), Ee();
  }
  function Y(t) {
    (this._fn = t),
      (this._cleanup = void 0),
      (this._sources = void 0),
      (this._nextBatchedEffect = void 0),
      (this._flags = G);
  }
  Y.prototype._callback = function () {
    let t = this._start();
    try {
      if (this._flags & j || this._fn === void 0) return;
      let e = this._fn();
      typeof e == "function" && (this._cleanup = e);
    } finally {
      t();
    }
  };
  Y.prototype._start = function () {
    if (this._flags & D) throw f("SignalCycleDetected");
    (this._flags |= D), (this._flags &= ~j), Ke(this), Be(this), ye();
    let t = E;
    return (E = this), on.bind(this, t);
  };
  Y.prototype._notify = function () {
    this._flags & B ||
      ((this._flags |= B), (this._nextBatchedEffect = z), (z = this));
  };
  Y.prototype._dispose = function () {
    (this._flags |= j), this._flags & D || Se(this);
  };
  function oe(t) {
    let e = new Y(t);
    try {
      e._callback();
    } catch (n) {
      throw (e._dispose(), n);
    }
    return e._dispose.bind(e);
  }
  function Je(t, e = !1) {
    let n = {};
    for (let r in t)
      if (t.hasOwnProperty(r)) {
        let i = t[r];
        if (i instanceof R) {
          if (e && r.startsWith("_")) continue;
          n[r] = i.value;
        } else n[r] = Je(i);
      }
    return n;
  }
  function ze(t, e, n = !1) {
    for (let r in e)
      if (e.hasOwnProperty(r)) {
        if (r.match(/\_\_+/)) throw f("InvalidSignalKey", { key: r });
        let i = e[r];
        if (i instanceof Object && !Array.isArray(i))
          t[r] || (t[r] = {}), ze(t[r], i, n);
        else {
          if (n && t[r]) continue;
          t[r] = new R(i);
        }
      }
  }
  function Xe(t, e) {
    for (let n in t)
      if (t.hasOwnProperty(n)) {
        let r = t[n];
        r instanceof R
          ? e(n, r)
          : Xe(r, (i, s) => {
              e(`${n}.${i}`, s);
            });
      }
  }
  function an(t, ...e) {
    let n = {};
    for (let r of e) {
      let i = r.split("."),
        s = t,
        o = n;
      for (let l = 0; l < i.length - 1; l++) {
        let c = i[l];
        if (!s[c]) return {};
        o[c] || (o[c] = {}), (s = s[c]), (o = o[c]);
      }
      let a = i[i.length - 1];
      o[a] = s[a];
    }
    return n;
  }
  var ae = class {
    #e = {};
    constructor() {}
    exists(e) {
      return !!this.signal(e);
    }
    signal(e) {
      let n = e.split("."),
        r = this.#e;
      for (let o = 0; o < n.length - 1; o++) {
        let a = n[o];
        if (!r[a]) return null;
        r = r[a];
      }
      let i = n[n.length - 1],
        s = r[i];
      if (!s) throw f("SignalNotFound", { path: e });
      return s;
    }
    setSignal(e, n) {
      let r = e.split("."),
        i = this.#e;
      for (let o = 0; o < r.length - 1; o++) {
        let a = r[o];
        i[a] || (i[a] = {}), (i = i[a]);
      }
      let s = r[r.length - 1];
      i[s] = n;
    }
    setComputed(e, n) {
      let r = je(() => n());
      this.setSignal(e, r);
    }
    value(e) {
      return this.signal(e)?.value;
    }
    setValue(e, n) {
      let r = this.upsert(e, n);
      r.value = n;
    }
    upsert(e, n) {
      let r = e.split("."),
        i = this.#e;
      for (let l = 0; l < r.length - 1; l++) {
        let c = r[l];
        i[c] || (i[c] = {}), (i = i[c]);
      }
      let s = r[r.length - 1],
        o = i[s];
      if (o)
        return (o.value === null || o.value === void 0) && (o.value = n), o;
      let a = new R(n);
      return (i[s] = a), a;
    }
    remove(...e) {
      for (let n of e) {
        let r = n.split("."),
          i = this.#e;
        for (let o = 0; o < r.length - 1; o++) {
          let a = r[o];
          if (!i[a]) return;
          i = i[a];
        }
        let s = r[r.length - 1];
        delete i[s];
      }
    }
    merge(e, n = !1) {
      ze(this.#e, e, n);
    }
    subset(...e) {
      return an(this.values(), ...e);
    }
    walk(e) {
      Xe(this.#e, e);
    }
    values(e = !1) {
      return Je(this.#e, e);
    }
    JSON(e = !0, n = !1) {
      let r = this.values(n);
      return e ? JSON.stringify(r, null, 2) : JSON.stringify(r);
    }
    toString() {
      return this.JSON();
    }
  };
  var le = class {
    #e = new ae();
    #i = [];
    #s = [];
    #n = {};
    #a = [];
    #t = new Map();
    get signals() {
      return this.#e;
    }
    get version() {
      return Oe;
    }
    load(...e) {
      e.forEach((n) => {
        let r;
        switch (n.type) {
          case 0:
            this.#s.push(n);
            break;
          case 2:
            let i = n;
            this.#a.push(i), (r = i.onGlobalInit);
            break;
          case 3:
            this.#n[n.name] = n;
            break;
          case 1:
            let s = n;
            this.#i.push(s), (r = s.onGlobalInit);
            break;
          default:
            throw f("InvalidPluginType", { name: n.name, type: n.type });
        }
        if (r) {
          let i = this;
          r({
            get signals() {
              return i.#e;
            },
            effect: (s) => oe(s),
            actions: this.#n,
            apply: this.apply.bind(this),
            cleanup: this.#r.bind(this),
          });
        }
      }),
        this.apply(document.body);
    }
    apply(e) {
      let n = new Set();
      this.#i.forEach((r, i) => {
        this.#o(e, (s) => {
          if (!("starIgnore" in s.dataset)) {
            i || this.#r(s);
            for (let o in s.dataset) {
              if (!o.startsWith(r.name)) continue;
              let a = o.slice(r.name.length),
                [l, ...c] = a.split(/\_\_+/),
                u = l.length > 0;
              u && (l = l[0].toLowerCase() + l.slice(1));
              let d = `${s.dataset[o]}` || "",
                p = d,
                m = p.length > 0,
                g = r.keyReq || 0;
              if (u) {
                if (g === 2) throw f(r.name + "KeyNotAllowed", { key: l });
              } else if (g === 1) throw f(r.name + "KeyRequired");
              let w = r.valReq || 0;
              if (m) {
                if (w === 2) throw f(r.name + "ValueNotAllowed", { value: p });
              } else if (w === 1) throw f(r.name + "ValueRequired");
              if (g === 3 || w === 3) {
                if (u && m) throw f(r.name + "KeyAndValueProvided");
                if (!u && !m) throw f(r.name + "KeyOrValueRequired");
              }
              s.id.length || (s.id = qe(s)), n.clear();
              let T = new Map();
              c.forEach((h) => {
                let [x, ..._] = h.split(".");
                T.set(Le(x), new Set(_.map((M) => M.toLowerCase())));
              });
              let A = [
                ...(r.macros?.pre || []),
                ...this.#s,
                ...(r.macros?.post || []),
              ];
              for (let h of A) n.has(h) || (n.add(h), (p = h.fn(p)));
              let y = this,
                b;
              b = {
                get signals() {
                  return y.#e;
                },
                effect: (h) => oe(h),
                apply: y.apply.bind(y),
                cleanup: y.#r.bind(y),
                actions: y.#n,
                genRX: () => this.#l(b, ...(r.argNames || [])),
                el: s,
                rawKey: o,
                rawValue: d,
                key: l,
                value: p,
                mods: T,
              };
              let v = r.onLoad(b);
              v &&
                (this.#t.has(s) || this.#t.set(s, { id: s.id, set: new Set() }),
                this.#t.get(s).set.add(v)),
                r?.removeOnLoad && delete s.dataset[o];
            }
          }
        });
      });
    }
    #l(e, ...n) {
      let r = e.value
          .split(/;|\n/)
          .map((g) => g.trim())
          .filter((g) => g != ""),
        i = r.length - 1;
      r[i].startsWith("return") || (r[i] = `return (${r[i]});`);
      let o = r.join(`
`),
        a = /(\w*)\(/gm,
        l = o.matchAll(a),
        c = new Set();
      for (let g of l) c.add(g[1]);
      let u = Object.keys(this.#n).filter((g) => c.has(g)),
        p = `${u.map((g) => `const ${g} = ctx.actions.${g}.fn;`).join(`
`)}return (()=> {${o}})()`,
        m = p.trim();
      u.forEach((g) => {
        m = m.replaceAll(g + "(", g + "(ctx,");
      });
      try {
        let g = n || [],
          w = new Function("ctx", ...g, m);
        return (...T) => w(e, ...T);
      } catch (g) {
        throw f("GeneratingExpressionFailed", { error: g, fnContent: p });
      }
    }
    #o(e, n) {
      if (!e || !(e instanceof HTMLElement || e instanceof SVGElement))
        return null;
      for (n(e), e = e.firstElementChild; e; )
        this.#o(e, n), (e = e.nextElementSibling);
    }
    #r(e) {
      let n = this.#t.get(e);
      if (n) {
        for (let r of n.set) r();
        this.#t.delete(e);
      }
    }
  };
  var Ye = new le();
  Ye.load(Ce, De, ke, Ie);
  var Ze = Ye;
  async function ln(t, e) {
    let n = t.getReader(),
      r;
    for (; !(r = await n.read()).done; ) e(r.value);
  }
  function un(t) {
    let e,
      n,
      r,
      i = !1;
    return function (o) {
      e === void 0 ? ((e = o), (n = 0), (r = -1)) : (e = fn(e, o));
      let a = e.length,
        l = 0;
      for (; n < a; ) {
        i && (e[n] === 10 && (l = ++n), (i = !1));
        let c = -1;
        for (; n < a && c === -1; ++n)
          switch (e[n]) {
            case 58:
              r === -1 && (r = n - l);
              break;
            case 13:
              i = !0;
            case 10:
              c = n;
              break;
          }
        if (c === -1) break;
        t(e.subarray(l, c), r), (l = n), (r = -1);
      }
      l === a ? (e = void 0) : l !== 0 && ((e = e.subarray(l)), (n -= l));
    };
  }
  function cn(t, e, n) {
    let r = Qe(),
      i = new TextDecoder();
    return function (o, a) {
      if (o.length === 0) n?.(r), (r = Qe());
      else if (a > 0) {
        let l = i.decode(o.subarray(0, a)),
          c = a + (o[a + 1] === 32 ? 2 : 1),
          u = i.decode(o.subarray(c));
        switch (l) {
          case "data":
            r.data = r.data
              ? r.data +
                `
` +
                u
              : u;
            break;
          case "event":
            r.event = u;
            break;
          case "id":
            t((r.id = u));
            break;
          case "retry":
            let d = parseInt(u, 10);
            isNaN(d) || e((r.retry = d));
            break;
        }
      }
    };
  }
  function fn(t, e) {
    let n = new Uint8Array(t.length + e.length);
    return n.set(t), n.set(e, t.length), n;
  }
  function Qe() {
    return { data: "", event: "", id: "", retry: void 0 };
  }
  var dn = "text/event-stream",
    mn = 1e3,
    et = "last-event-id";
  function tt(
    t,
    {
      signal: e,
      headers: n,
      onopen: r,
      onmessage: i,
      onclose: s,
      onerror: o,
      openWhenHidden: a,
      fetch: l,
      retryScaler: c = 2,
      retryMaxWaitMs: u = 3e4,
      retryMaxCount: d = 10,
      ...p
    }
  ) {
    return new Promise((m, g) => {
      let w = 0,
        T = { ...n };
      T.accept || (T.accept = dn);
      let A;
      function y() {
        A.abort(), document.hidden || M();
      }
      a || document.addEventListener("visibilitychange", y);
      let b = mn,
        v = 0;
      function h() {
        document.removeEventListener("visibilitychange", y),
          window.clearTimeout(v),
          A.abort();
      }
      e?.addEventListener("abort", () => {
        h(), m();
      });
      let x = l ?? window.fetch,
        _ = r ?? function () {};
      async function M() {
        A = new AbortController();
        try {
          let C = await x(t, { ...p, headers: T, signal: A.signal });
          await _(C),
            await ln(
              C.body,
              un(
                cn(
                  (N) => {
                    N ? (T[et] = N) : delete T[et];
                  },
                  (N) => {
                    b = N;
                  },
                  i
                )
              )
            ),
            s?.(),
            h(),
            m();
        } catch (C) {
          if (!A.signal.aborted)
            try {
              let N = o?.(C) ?? b;
              window.clearTimeout(v),
                (v = window.setTimeout(M, N)),
                (b *= c),
                (b = Math.min(b, u)),
                w++,
                w >= d
                  ? (h(),
                    g(
                      f("SSE_MAX_RETRIES", {
                        retryInterval: b,
                        retryMaxCount: d,
                        ...p,
                      })
                    ))
                  : console.error(
                      `Datastar failed to reach ${
                        p.method
                      }:${t.toString()} retry in ${N}ms`
                    );
            } catch (N) {
              h(), g(N);
            }
        }
      }
      M();
    });
  }
  var K = `${L}-sse`,
    Te = `${L}-settling`,
    W = `${L}-swapping`,
    ue = "started",
    ce = "finished";
  function V(t, e) {
    document.addEventListener(K, (n) => {
      if (n.detail.type != t) return;
      let { argsRaw: r } = n.detail;
      e(r);
    });
  }
  function Ae(t, e) {
    document.dispatchEvent(
      new CustomEvent(K, { detail: { type: t, argsRaw: e } })
    );
  }
  var nt = (t) => `${t}`.includes("text/event-stream"),
    rt = {
      type: 3,
      name: "sse",
      fn: async (t, e, n) => {
        let {
            el: { id: r },
            signals: i,
          } = t,
          {
            method: s,
            headers: o,
            includeLocal: a,
            openWhenHidden: l,
            retryScaler: c,
            retryMaxWaitMs: u,
            retryMaxCount: d,
            abort: p,
          } = Object.assign(
            {
              method: "GET",
              headers: {},
              includeLocal: !1,
              openWhenHidden: !1,
              retryScaler: 2,
              retryMaxWaitMs: 3e4,
              retryMaxCount: 10,
              abort: void 0,
            },
            n
          ),
          m = s.toUpperCase();
        try {
          if ((Ae(ue, { elId: r }), !e?.length)) throw f("NoUrlProvided");
          let g = Object.assign(
              { "Content-Type": "application/json", [Ve]: !0 },
              o
            ),
            w = {
              method: m,
              headers: g,
              openWhenHidden: l,
              retryScaler: c,
              retryMaxWaitMs: u,
              retryMaxCount: d,
              signal: p,
              onmessage: (y) => {
                if (!y.event.startsWith(L)) return;
                let b = y.event,
                  v = {},
                  h = y.data.split(`
`);
                for (let _ of h) {
                  let M = _.indexOf(" "),
                    C = _.slice(0, M),
                    N = v[C];
                  N || ((N = []), (v[C] = N));
                  let te = _.slice(M + 1).trim();
                  N.push(te);
                }
                let x = {};
                for (let [_, M] of Object.entries(v))
                  x[_] = M.join(`
`);
                Ae(b, x);
              },
              onerror: (y) => {
                if (nt(y)) throw f("InvalidContentType", { url: e, error: y });
                y && console.error(y.message);
              },
            },
            T = new URL(e, window.location.origin),
            A = i.JSON(!1, !a);
          if (m === "GET") {
            let y = new URLSearchParams(T.search);
            y.set(L, A), (T.search = y.toString());
          } else w.body = A;
          try {
            await tt(T.toString(), w);
          } catch (y) {
            if (!nt(y))
              throw f("SseFetchFailed", { method: m, url: e, error: y });
          }
        } finally {
          Ae(ce, { elId: r });
        }
      },
    };
  var pn = `${L}-indicator`,
    Zr = `${pn}-loading`,
    it = {
      type: 1,
      name: "indicator",
      keyReq: 3,
      valReq: 3,
      onLoad: ({ value: t, signals: e, el: n, key: r }) => {
        let i = r || t,
          s = e.upsert(i, !1),
          o = (a) => {
            let {
              type: l,
              argsRaw: { elId: c },
            } = a.detail;
            if (c === n.id)
              switch (l) {
                case ue:
                  s.value = !0;
                  break;
                case ce:
                  s.value = !1;
                  break;
              }
          };
        return (
          document.addEventListener(K, o),
          () => {
            document.removeEventListener(K, o);
          }
        );
      },
    };
  var st = {
    type: 2,
    name: I.ExecuteScript,
    onGlobalInit: async () => {
      V(
        I.ExecuteScript,
        ({ autoRemove: t = `${!0}`, attributes: e = Fe, script: n }) => {
          let r = O(t);
          if (!n?.length) throw f("NoScriptProvided");
          let i = document.createElement("script");
          e
            .split(
              `
`
            )
            .forEach((s) => {
              let o = s.indexOf(" "),
                a = o ? s.slice(0, o) : s,
                l = o ? s.slice(o) : "";
              i.setAttribute(a.trim(), l.trim());
            }),
            (i.text = n),
            document.head.appendChild(i),
            r && i.remove();
        }
      );
    },
  };
  var Z = document,
    J = !!Z.startViewTransition;
  var de = new WeakSet();
  function ut(t, e, n = {}) {
    t instanceof Document && (t = t.documentElement);
    let r;
    typeof e == "string" ? (r = En(e)) : (r = e);
    let i = Sn(r),
      s = vn(t, i, n);
    return ct(t, i, s);
  }
  function ct(t, e, n) {
    if (n.head.block) {
      let r = t.querySelector("head"),
        i = e.querySelector("head");
      if (r && i) {
        let s = dt(i, r, n);
        Promise.all(s).then(() => {
          ct(t, e, Object.assign(n, { head: { block: !1, ignore: !0 } }));
        });
        return;
      }
    }
    if (n.morphStyle === "innerHTML") return ft(e, t, n), t.children;
    if (n.morphStyle === "outerHTML" || n.morphStyle == null) {
      let r = An(e, t, n);
      if (!r) throw f("NoBestMatchFound", { old: t, new: e });
      let i = r?.previousSibling,
        s = r?.nextSibling,
        o = me(t, r, n);
      return r ? Tn(i, o, s) : [];
    } else throw f("InvalidMorphStyle", { style: n.morphStyle });
  }
  function me(t, e, n) {
    if (!(n.ignoreActive && t === document.activeElement))
      if (e == null) {
        if (n.callbacks.beforeNodeRemoved(t) === !1) return;
        t.remove(), n.callbacks.afterNodeRemoved(t);
        return;
      } else {
        if (pe(t, e))
          return n.callbacks.beforeNodeMorphed(t, e) === !1
            ? void 0
            : ((t instanceof HTMLHeadElement && n.head.ignore) ||
                (e instanceof HTMLHeadElement &&
                t instanceof HTMLHeadElement &&
                n.head.style !== k.Morph
                  ? dt(e, t, n)
                  : (hn(e, t), ft(e, t, n))),
              n.callbacks.afterNodeMorphed(t, e),
              t);
        if (
          n.callbacks.beforeNodeRemoved(t) === !1 ||
          n.callbacks.beforeNodeAdded(e) === !1
        )
          return;
        if (!t.parentElement) throw f("NoParentElementFound", { oldNode: t });
        return (
          t.parentElement.replaceChild(e, t),
          n.callbacks.afterNodeAdded(e),
          n.callbacks.afterNodeRemoved(t),
          e
        );
      }
  }
  function ft(t, e, n) {
    let r = t.firstChild,
      i = e.firstChild,
      s;
    for (; r; ) {
      if (((s = r), (r = s.nextSibling), i == null)) {
        if (n.callbacks.beforeNodeAdded(s) === !1) return;
        e.appendChild(s), n.callbacks.afterNodeAdded(s), U(n, s);
        continue;
      }
      if (mt(s, i, n)) {
        me(i, s, n), (i = i.nextSibling), U(n, s);
        continue;
      }
      let o = bn(t, e, s, i, n);
      if (o) {
        (i = ot(i, o, n)), me(o, s, n), U(n, s);
        continue;
      }
      let a = yn(t, s, i, n);
      if (a) {
        (i = ot(i, a, n)), me(a, s, n), U(n, s);
        continue;
      }
      if (n.callbacks.beforeNodeAdded(s) === !1) return;
      e.insertBefore(s, i), n.callbacks.afterNodeAdded(s), U(n, s);
    }
    for (; i !== null; ) {
      let o = i;
      (i = i.nextSibling), pt(o, n);
    }
  }
  function hn(t, e) {
    let n = t.nodeType;
    if (n === 1) {
      for (let r of t.attributes)
        e.getAttribute(r.name) !== r.value && e.setAttribute(r.name, r.value);
      for (let r of e.attributes)
        t.hasAttribute(r.name) || e.removeAttribute(r.name);
    }
    if (
      ((n === Node.COMMENT_NODE || n === Node.TEXT_NODE) &&
        e.nodeValue !== t.nodeValue &&
        (e.nodeValue = t.nodeValue),
      t instanceof HTMLInputElement &&
        e instanceof HTMLInputElement &&
        t.type !== "file")
    )
      (e.value = t.value || ""),
        fe(t, e, "value"),
        fe(t, e, "checked"),
        fe(t, e, "disabled");
    else if (t instanceof HTMLOptionElement) fe(t, e, "selected");
    else if (
      t instanceof HTMLTextAreaElement &&
      e instanceof HTMLTextAreaElement
    ) {
      let r = t.value,
        i = e.value;
      r !== i && (e.value = r),
        e.firstChild &&
          e.firstChild.nodeValue !== r &&
          (e.firstChild.nodeValue = r);
    }
  }
  function fe(t, e, n) {
    let r = t.getAttribute(n),
      i = e.getAttribute(n);
    r !== i && (r ? e.setAttribute(n, r) : e.removeAttribute(n));
  }
  function dt(t, e, n) {
    let r = [],
      i = [],
      s = [],
      o = [],
      a = n.head.style,
      l = new Map();
    for (let u of t.children) l.set(u.outerHTML, u);
    for (let u of e.children) {
      let d = l.has(u.outerHTML),
        p = n.head.shouldReAppend(u),
        m = n.head.shouldPreserve(u);
      d || m
        ? p
          ? i.push(u)
          : (l.delete(u.outerHTML), s.push(u))
        : a === k.Append
        ? p && (i.push(u), o.push(u))
        : n.head.shouldRemove(u) !== !1 && i.push(u);
    }
    o.push(...l.values());
    let c = [];
    for (let u of o) {
      let d = document
        .createRange()
        .createContextualFragment(u.outerHTML).firstChild;
      if (!d) throw f("NewElementCouldNotBeCreated", { newNode: u });
      if (n.callbacks.beforeNodeAdded(d)) {
        if (d.hasAttribute("href") || d.hasAttribute("src")) {
          let p,
            m = new Promise((g) => {
              p = g;
            });
          d.addEventListener("load", function () {
            p(void 0);
          }),
            c.push(m);
        }
        e.appendChild(d), n.callbacks.afterNodeAdded(d), r.push(d);
      }
    }
    for (let u of i)
      n.callbacks.beforeNodeRemoved(u) !== !1 &&
        (e.removeChild(u), n.callbacks.afterNodeRemoved(u));
    return n.head.afterHeadMorphed(e, { added: r, kept: s, removed: i }), c;
  }
  function F() {}
  function vn(t, e, n) {
    return {
      target: t,
      newContent: e,
      config: n,
      morphStyle: n.morphStyle,
      ignoreActive: n.ignoreActive,
      idMap: xn(t, e),
      deadIds: new Set(),
      callbacks: Object.assign(
        {
          beforeNodeAdded: F,
          afterNodeAdded: F,
          beforeNodeMorphed: F,
          afterNodeMorphed: F,
          beforeNodeRemoved: F,
          afterNodeRemoved: F,
        },
        n.callbacks
      ),
      head: Object.assign(
        {
          style: "merge",
          shouldPreserve: (r) => r.getAttribute("im-preserve") === "true",
          shouldReAppend: (r) => r.getAttribute("im-re-append") === "true",
          shouldRemove: F,
          afterHeadMorphed: F,
        },
        n.head
      ),
    };
  }
  function mt(t, e, n) {
    return !t || !e
      ? !1
      : t.nodeType === e.nodeType && t.tagName === e.tagName
      ? t?.id?.length && t.id === e.id
        ? !0
        : Q(n, t, e) > 0
      : !1;
  }
  function pe(t, e) {
    return !t || !e ? !1 : t.nodeType === e.nodeType && t.tagName === e.tagName;
  }
  function ot(t, e, n) {
    for (; t !== e; ) {
      let r = t;
      if (((t = t?.nextSibling), !r))
        throw f("NoTemporaryNodeFound", { startInclusive: t, endExclusive: e });
      pt(r, n);
    }
    return U(n, e), e.nextSibling;
  }
  function bn(t, e, n, r, i) {
    let s = Q(i, n, e),
      o = null;
    if (s > 0) {
      o = r;
      let a = 0;
      for (; o != null; ) {
        if (mt(n, o, i)) return o;
        if (((a += Q(i, o, t)), a > s)) return null;
        o = o.nextSibling;
      }
    }
    return o;
  }
  function yn(t, e, n, r) {
    let i = n,
      s = e.nextSibling,
      o = 0;
    for (; i && s; ) {
      if (Q(r, i, t) > 0) return null;
      if (pe(e, i)) return i;
      if (pe(s, i) && (o++, (s = s.nextSibling), o >= 2)) return null;
      i = i.nextSibling;
    }
    return i;
  }
  var at = new DOMParser();
  function En(t) {
    let e = t.replace(/<svg(\s[^>]*>|>)([\s\S]*?)<\/svg>/gim, "");
    if (e.match(/<\/html>/) || e.match(/<\/head>/) || e.match(/<\/body>/)) {
      let n = at.parseFromString(t, "text/html");
      if (e.match(/<\/html>/)) return de.add(n), n;
      {
        let r = n.firstChild;
        return r ? (de.add(r), r) : null;
      }
    } else {
      let r = at
        .parseFromString(`<body><template>${t}</template></body>`, "text/html")
        .body.querySelector("template")?.content;
      if (!r) throw f("NoContentFound", { newContent: t });
      return de.add(r), r;
    }
  }
  function Sn(t) {
    if (t == null) return document.createElement("div");
    if (de.has(t)) return t;
    if (t instanceof Node) {
      let e = document.createElement("div");
      return e.append(t), e;
    } else {
      let e = document.createElement("div");
      for (let n of [...t]) e.append(n);
      return e;
    }
  }
  function Tn(t, e, n) {
    let r = [],
      i = [];
    for (; t; ) r.push(t), (t = t.previousSibling);
    for (; r.length > 0; ) {
      let s = r.pop();
      i.push(s), e?.parentElement?.insertBefore(s, e);
    }
    for (i.push(e); n; ) r.push(n), i.push(n), (n = n.nextSibling);
    for (; r.length; ) e?.parentElement?.insertBefore(r.pop(), e.nextSibling);
    return i;
  }
  function An(t, e, n) {
    let r = t.firstChild,
      i = r,
      s = 0;
    for (; r; ) {
      let o = _n(r, e, n);
      o > s && ((i = r), (s = o)), (r = r.nextSibling);
    }
    return i;
  }
  function _n(t, e, n) {
    return pe(t, e) ? 0.5 + Q(n, t, e) : 0;
  }
  function pt(t, e) {
    U(e, t),
      e.callbacks.beforeNodeRemoved(t) !== !1 &&
        (t.remove(), e.callbacks.afterNodeRemoved(t));
  }
  function wn(t, e) {
    return !t.deadIds.has(e);
  }
  function Rn(t, e, n) {
    return t.idMap.get(n)?.has(e) || !1;
  }
  function U(t, e) {
    let n = t.idMap.get(e);
    if (n) for (let r of n) t.deadIds.add(r);
  }
  function Q(t, e, n) {
    let r = t.idMap.get(e);
    if (!r) return 0;
    let i = 0;
    for (let s of r) wn(t, s) && Rn(t, s, n) && ++i;
    return i;
  }
  function lt(t, e) {
    let n = t.parentElement,
      r = t.querySelectorAll("[id]");
    for (let i of r) {
      let s = i;
      for (; s !== n && s; ) {
        let o = e.get(s);
        o == null && ((o = new Set()), e.set(s, o)),
          o.add(i.id),
          (s = s.parentElement);
      }
    }
  }
  function xn(t, e) {
    let n = new Map();
    return lt(t, n), lt(e, n), n;
  }
  var ht = {
    type: 2,
    name: I.MergeFragments,
    onGlobalInit: async (t) => {
      let e = document.createElement("template");
      V(
        I.MergeFragments,
        ({
          fragments: n = "<div></div>",
          selector: r = "",
          mergeMode: i = He,
          settleDuration: s = `${300}`,
          useViewTransition: o = `${!1}`,
        }) => {
          let a = parseInt(s),
            l = O(o);
          (e.innerHTML = n.trim()),
            [...e.content.children].forEach((u) => {
              if (!(u instanceof Element)) throw f("NoFragmentsFound");
              let d = r || `#${u.getAttribute("id")}`,
                p = [...(document.querySelectorAll(d) || [])];
              if (!p.length) throw f("NoTargetsFound", { selectorOrID: d });
              J && l
                ? Z.startViewTransition(() => gt(t, i, a, u, p))
                : gt(t, i, a, u, p);
            });
        }
      );
    },
  };
  function gt(t, e, n, r, i) {
    for (let s of i) {
      s.classList.add(W);
      let o = s.outerHTML,
        a = s;
      switch (e) {
        case k.Morph:
          let u = ut(a, r, {
            callbacks: { beforeNodeRemoved: (d, p) => (t.cleanup(d), !0) },
          });
          if (!u?.length) throw f("MorphFailed");
          a = u[0];
          break;
        case k.Inner:
          a.innerHTML = r.innerHTML;
          break;
        case k.Outer:
          a.replaceWith(r);
          break;
        case k.Prepend:
          a.prepend(r);
          break;
        case k.Append:
          a.append(r);
          break;
        case k.Before:
          a.before(r);
          break;
        case k.After:
          a.after(r);
          break;
        case k.UpsertAttributes:
          r.getAttributeNames().forEach((d) => {
            let p = r.getAttribute(d);
            a.setAttribute(d, p);
          });
          break;
        default:
          throw f("InvalidMergeMode", { mergeMode: e });
      }
      t.cleanup(a);
      let l = a.classList;
      l.add(W),
        t.apply(document.body),
        setTimeout(() => {
          s.classList.remove(W), l.remove(W);
        }, n);
      let c = a.outerHTML;
      o !== c &&
        (l.add(Te),
        setTimeout(() => {
          l.remove(Te);
        }, n));
    }
  }
  var vt = {
    type: 2,
    name: I.MergeSignals,
    onGlobalInit: async (t) => {
      V(I.MergeSignals, ({ signals: e = "{}", onlyIfMissing: n = `${!1}` }) => {
        let { signals: r } = t,
          i = O(n);
        r.merge(ne(e), i), t.apply(document.body);
      });
    },
  };
  var bt = {
    type: 2,
    name: I.RemoveFragments,
    onGlobalInit: async () => {
      V(
        I.RemoveFragments,
        ({
          selector: t,
          settleDuration: e = `${300}`,
          useViewTransition: n = `${!1}`,
        }) => {
          if (!t.length) throw f("NoSelectorProvided");
          let r = parseInt(e),
            i = O(n),
            s = document.querySelectorAll(t),
            o = () => {
              for (let a of s) a.classList.add(W);
              setTimeout(() => {
                for (let a of s) a.remove();
              }, r);
            };
          J && i ? Z.startViewTransition(() => o()) : o();
        }
      );
    },
  };
  var yt = {
    type: 2,
    name: I.RemoveSignals,
    onGlobalInit: async (t) => {
      V(I.RemoveSignals, ({ paths: e = "" }) => {
        let n = e
          .split(
            `
`
          )
          .map((r) => r.trim());
        if (!n?.length) throw f("NoPathsProvided");
        t.signals.remove(...n), t.apply(document.body);
      });
    },
  };
  var Et = "once",
    St = "half",
    Tt = "full",
    At = {
      type: 1,
      name: "intersects",
      keyReq: 2,
      mods: new Set([Et, St, Tt]),
      onLoad: ({ el: t, rawKey: e, mods: n, genRX: r }) => {
        let i = { threshold: 0 };
        n.has(Tt) ? (i.threshold = 1) : n.has(St) && (i.threshold = 0.5);
        let s = r(),
          o = new IntersectionObserver((a) => {
            a.forEach((l) => {
              l.isIntersecting &&
                (s(), n.has(Et) && (o.disconnect(), delete t.dataset[e]));
            });
          }, i);
        return o.observe(t), () => o.disconnect();
      },
    };
  var _t = "session",
    wt = {
      type: 1,
      name: "persist",
      mods: new Set([_t]),
      onLoad: ({ key: t, value: e, signals: n, effect: r, mods: i }) => {
        t === "" && (t = L);
        let s = i.has(_t) ? sessionStorage : localStorage,
          o = e.split(/\s+/).filter((c) => c !== ""),
          a = () => {
            let c = s.getItem(t) || "{}",
              u = JSON.parse(c);
            n.merge(u);
          },
          l = () => {
            let c;
            o.length ? (c = n.subset(...o)) : (c = n.values()),
              s.setItem(t, JSON.stringify(c));
          };
        return (
          a(),
          r(() => {
            l();
          })
        );
      },
    };
  var Rt = {
    type: 1,
    name: "replaceUrl",
    keyReq: 2,
    valReq: 1,
    onLoad: ({ effect: t, genRX: e }) => {
      let n = e();
      return t(() => {
        let r = n(),
          i = window.location.href,
          s = new URL(r, i).toString();
        window.history.replaceState({}, "", s);
      });
    },
  };
  var ge = "smooth",
    Re = "instant",
    xe = "auto",
    xt = "hstart",
    Mt = "hcenter",
    Nt = "hend",
    Pt = "hnearest",
    It = "vstart",
    Lt = "vcenter",
    kt = "vend",
    Ct = "vnearest",
    Nn = "focus",
    he = "center",
    Dt = "start",
    Vt = "end",
    Ot = "nearest",
    Ft = {
      type: 1,
      name: "scrollIntoView",
      keyReq: 2,
      valReq: 2,
      mods: new Set([ge, Re, xe, xt, Mt, Nt, Pt, It, Lt, kt, Ct, Nn]),
      onLoad: ({ el: t, mods: e, rawKey: n }) => {
        t.tabIndex || t.setAttribute("tabindex", "0");
        let r = { behavior: ge, block: he, inline: he };
        if (
          (e.has(ge) && (r.behavior = ge),
          e.has(Re) && (r.behavior = Re),
          e.has(xe) && (r.behavior = xe),
          e.has(xt) && (r.inline = Dt),
          e.has(Mt) && (r.inline = he),
          e.has(Nt) && (r.inline = Vt),
          e.has(Pt) && (r.inline = Ot),
          e.has(It) && (r.block = Dt),
          e.has(Lt) && (r.block = he),
          e.has(kt) && (r.block = Vt),
          e.has(Ct) && (r.block = Ot),
          !(t instanceof HTMLElement || t instanceof SVGElement))
        )
          throw f("NotHtmlSvgElement, el");
        return (
          t.tabIndex || t.setAttribute("tabindex", "0"),
          t.scrollIntoView(r),
          e.has("focus") && t.focus(),
          delete t.dataset[n],
          () => {}
        );
      },
    };
  var Ht = "none",
    qt = "display",
    Wt = {
      type: 1,
      name: "show",
      keyReq: 2,
      valReq: 1,
      onLoad: ({ el: { style: t }, genRX: e, effect: n }) => {
        let r = e();
        return n(async () => {
          r()
            ? t.display === Ht && t.removeProperty(qt)
            : t.setProperty(qt, Ht);
        });
      },
    };
  var Me = "view-transition",
    Ut = {
      type: 1,
      name: Me,
      keyReq: 2,
      valReq: 1,
      onGlobalInit() {
        let t = !1;
        if (
          (document.head.childNodes.forEach((e) => {
            e instanceof HTMLMetaElement && e.name === Me && (t = !0);
          }),
          !t)
        ) {
          let e = document.createElement("meta");
          (e.name = Me),
            (e.content = "same-origin"),
            document.head.appendChild(e);
        }
      },
      onLoad: ({ effect: t, el: e, genRX: n }) => {
        if (!J) {
          console.error("Browser does not support view transitions");
          return;
        }
        let r = n();
        return t(() => {
          let i = r();
          if (!i?.length) return;
          let s = e.style;
          s.viewTransitionName = i;
        });
      },
    };
  var $t = {
    type: 1,
    name: "attributes",
    valReq: 1,
    onLoad: ({ el: t, genRX: e, key: n, effect: r }) => {
      let i = e();
      return n === ""
        ? r(async () => {
            let s = i();
            Object.entries(s).forEach(([o, a]) => {
              t.setAttribute(o, a);
            });
          })
        : ((n = $(n)),
          r(async () => {
            let s = !1;
            try {
              s = i();
            } catch {}
            let o;
            typeof s == "string" ? (o = s) : (o = JSON.stringify(s)),
              !o || o === "false" || o === "null" || o === "undefined"
                ? t.removeAttribute(n)
                : t.setAttribute(n, o);
          }));
    },
  };
  var Pn = /^data:(?<mime>[^;]+);base64,(?<contents>.*)$/,
    Bt = ["change", "input", "keydown"],
    Gt = {
      type: 1,
      name: "bind",
      keyReq: 3,
      valReq: 3,
      onLoad: (t) => {
        let { el: e, value: n, key: r, signals: i, effect: s } = t,
          o = r || n,
          a = () => {},
          l = () => {};
        if (typeof o != "string") throw f("InvalidExpression");
        let c = e.tagName.toLowerCase(),
          u = "",
          d = c.includes("input"),
          p = e.getAttribute("type"),
          m = c.includes("checkbox") || (d && p === "checkbox");
        m && (u = !1), d && p === "number" && (u = 0);
        let w = c.includes("select"),
          T = c.includes("radio") || (d && p === "radio"),
          A = d && p === "file";
        T && (e.getAttribute("name")?.length || e.setAttribute("name", o)),
          i.upsert(o, u),
          (a = () => {
            let b = "value" in e,
              v = i.value(o),
              h = `${v}`;
            if (m || T) {
              let x = e;
              m
                ? (x.checked = !!v || v === "true")
                : T && (x.checked = h === x.value);
            } else if (!A)
              if (w) {
                let x = e;
                x.multiple
                  ? Array.from(x.options).forEach((_) => {
                      _?.disabled ||
                        (Array.isArray(v) || typeof v == "string"
                          ? (_.selected = v.includes(_.value))
                          : typeof v == "number"
                          ? (_.selected = v === Number(_.value))
                          : (_.selected = v));
                    })
                  : (x.value = h);
              } else b ? (e.value = h) : e.setAttribute("value", h);
          }),
          (l = async () => {
            if (A) {
              let h = [...(e?.files || [])],
                x = [],
                _ = [],
                M = [];
              await Promise.all(
                h.map(
                  (te) =>
                    new Promise((tn) => {
                      let H = new FileReader();
                      (H.onload = () => {
                        if (typeof H.result != "string")
                          throw f("InvalidFileResultType", {
                            type: typeof H.result,
                          });
                        let ve = H.result.match(Pn);
                        if (!ve?.groups)
                          throw f("InvalidDataUri", { result: H.result });
                        x.push(ve.groups.contents),
                          _.push(ve.groups.mime),
                          M.push(te.name);
                      }),
                        (H.onloadend = () => tn(void 0)),
                        H.readAsDataURL(te);
                    })
                )
              ),
                i.setValue(o, x);
              let C = `${o}Mimes`,
                N = `${o}Names`;
              C in i && i.upsert(C, _), N in i && i.upsert(N, M);
              return;
            }
            let b = i.value(o),
              v = e || e;
            if (typeof b == "number") {
              let h = Number(v.value || v.getAttribute("value"));
              i.setValue(o, h);
            } else if (typeof b == "string") {
              let h = v.value || v.getAttribute("value") || "";
              i.setValue(o, h);
            } else if (typeof b == "boolean")
              if (m) {
                let h = v.checked || v.getAttribute("checked") === "true";
                i.setValue(o, h);
              } else {
                let h = !!(v.value || v.getAttribute("value"));
                i.setValue(o, h);
              }
            else if (!(typeof b > "u"))
              if (Array.isArray(b))
                if (w) {
                  let _ = [...e.selectedOptions]
                    .filter((M) => M.selected)
                    .map((M) => M.value);
                  i.setValue(o, _);
                } else {
                  let h = JSON.stringify(v.value.split(","));
                  i.setValue(o, h);
                }
              else throw f("UnsupportedSignalType", { current: typeof b });
          }),
          Bt.forEach((b) => e.addEventListener(b, l));
        let y = s(() => a());
        return () => {
          y(),
            Bt.forEach((b) => {
              e.removeEventListener(b, l);
            });
        };
      },
    };
  var jt = {
    type: 1,
    name: "class",
    valReq: 1,
    onLoad: ({ key: t, el: e, genRX: n, effect: r }) => {
      let i = e.classList,
        s = n();
      return r(() => {
        if (t === "") {
          let o = s();
          for (let [a, l] of Object.entries(o)) {
            let c = a.split(/\s+/);
            l ? i.add(...c) : i.remove(...c);
          }
        } else {
          let o = s(),
            a = $(t);
          o ? i.add(a) : i.remove(a);
        }
      });
    },
  };
  function Ne(t) {
    if (!t || t.size <= 0) return 0;
    for (let e of t) {
      if (e.endsWith("ms")) return Number(e.replace("ms", ""));
      if (e.endsWith("s")) return Number(e.replace("s", "")) * 1e3;
      try {
        return parseFloat(e);
      } catch {}
    }
    return 0;
  }
  function ee(t, e, n = !1) {
    return t ? t.has(e.toLowerCase()) : n;
  }
  function Kt(t, e, n = !1, r = !0) {
    let i = -1,
      s = () => i && clearTimeout(i);
    return function (...a) {
      s(),
        n && !i && t(...a),
        (i = setTimeout(() => {
          r && t(...a), s();
        }, e));
    };
  }
  function Jt(t, e, n = !0, r = !1) {
    let i = !1;
    return function (...o) {
      i ||
        (n && t(...o),
        (i = !0),
        setTimeout(() => {
          (i = !1), r && t(...o);
        }, e));
    };
  }
  var Pe = new Map(),
    In = "evt",
    zt = {
      type: 1,
      name: "on",
      keyReq: 1,
      valReq: 1,
      argNames: [In],
      macros: {
        pre: [
          {
            type: 0,
            name: "evtEsc",
            fn: (t) => t.replaceAll(/evt.([\w\.]+)value/gm, "EVT_$1_VALUE"),
          },
        ],
        post: [
          {
            type: 0,
            name: "evtUnesc",
            fn: (t) => t.replaceAll(/EVT_([\w\.]+)_VALUE/gm, "evt.$1value"),
          },
        ],
      },
      onLoad: ({ el: t, key: e, genRX: n, mods: r, signals: i, effect: s }) => {
        let o = n(),
          a = t;
        r.has("window") && (a = window);
        let l = (m) => {
            m &&
              (r.has("prevent") && m.preventDefault(),
              r.has("stop") && m.stopPropagation()),
              o(m);
          },
          c = r.get("debounce");
        if (c) {
          let m = Ne(c),
            g = ee(c, "leading", !1),
            w = !ee(c, "notrail", !1);
          l = Kt(l, m, g, w);
        }
        let u = r.get("throttle");
        if (u) {
          let m = Ne(u),
            g = !ee(u, "noleading", !1),
            w = ee(u, "trail", !1);
          l = Jt(l, m, g, w);
        }
        let d = { capture: !0, passive: !1, once: !1 };
        r.has("capture") || (d.capture = !1),
          r.has("passive") && (d.passive = !0),
          r.has("once") && (d.once = !0);
        let p = $(e).toLowerCase();
        switch (p) {
          case "load":
            return l(), delete t.dataset.onLoad, () => {};
          case "raf":
            let m,
              g = () => {
                l(), (m = requestAnimationFrame(g));
              };
            return (
              (m = requestAnimationFrame(g)),
              () => {
                m && cancelAnimationFrame(m);
              }
            );
          case "signals-change":
            return (
              We(t, () => {
                Pe.delete(t.id);
              }),
              s(() => {
                let T = r.has("remote"),
                  A = i.JSON(!1, T);
                (Pe.get(t.id) || "") !== A && (Pe.set(t.id, A), l());
              })
            );
          default:
            if (r.has("outside")) {
              a = document;
              let T = l,
                A = !1;
              l = (b) => {
                let v = b?.target;
                if (!v) return;
                let h = t.id === v.id;
                h && A && (A = !1), !h && !A && (T(b), (A = !0));
              };
            }
            return (
              a.addEventListener(p, l, d),
              () => {
                a.removeEventListener(p, l);
              }
            );
        }
      },
    };
  var Xt = {
    type: 1,
    name: "ref",
    keyReq: 3,
    valReq: 3,
    onLoad: ({ el: t, key: e, value: n, signals: r }) => {
      let i = e || n;
      return r.upsert(i, t), () => r.setValue(i, null);
    },
  };
  var Yt = {
    type: 1,
    name: "text",
    keyReq: 2,
    valReq: 1,
    onLoad: (t) => {
      let { el: e, genRX: n, effect: r } = t,
        i = n();
      return (
        e instanceof HTMLElement || f("NotHtmlElement"),
        r(() => {
          let s = i(t);
          e.textContent = `${s}`;
        })
      );
    },
  };
  var { round: Ln, max: kn, min: Cn } = Math,
    Zt = {
      type: 3,
      name: "fit",
      fn: (t, e, n, r, i, s, o = !1, a = !1) => {
        let l = ((e - n) / (r - n)) * (s - i) + i;
        return a && (l = Ln(l)), o && (l = kn(i, Cn(s, l))), l;
      },
    };
  var Qt = {
    type: 3,
    name: "setAll",
    fn: ({ signals: t }, e, n) => {
      t.walk((r, i) => {
        r.startsWith(e) && (i.value = n);
      });
    },
  };
  var en = {
    type: 3,
    name: "toggleAll",
    fn: ({ signals: t }, e) => {
      t.walk((n, r) => {
        n.startsWith(e) && (r.value = !r.value);
      });
    },
  };
  Ze.load(
    rt,
    it,
    st,
    ht,
    vt,
    bt,
    yt,
    At,
    wt,
    Rt,
    Ft,
    Wt,
    Ut,
    $t,
    Gt,
    jt,
    zt,
    Xt,
    Yt,
    Zt,
    Qt,
    en
  );
})();
//# sourceMappingURL=datastar-0-21-3-a97359219fc18db5.js.map
