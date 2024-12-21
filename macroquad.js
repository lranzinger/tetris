"use strict";
const version = 2,
  canvas = document.querySelector("#glcanvas");
var gl,
  clipboard = null,
  wasm_memory,
  animation_frame_timeout,
  FS,
  GL,
  Module,
  wasm_exports,
  emscripten_shaders_hack,
  importObject,
  plugins = [],
  high_dpi = !1,
  blocking_event_loop = !1;
function init_webgl(e) {
  if (e == 1) {
    gl = canvas.getContext("webgl");
    function t(e) {
      var t = e.getExtension("OES_vertex_array_object");
      t
        ? ((e.createVertexArray = function () {
          return t.createVertexArrayOES();
        }),
          (e.deleteVertexArray = function (e) {
            t.deleteVertexArrayOES(e);
          }),
          (e.bindVertexArray = function (e) {
            t.bindVertexArrayOES(e);
          }),
          (e.isVertexArray = function (e) {
            return t.isVertexArrayOES(e);
          }))
        : alert("Unable to get OES_vertex_array_object extension");
    }
    function n(e) {
      var t = e.getExtension("ANGLE_instanced_arrays");
      t &&
        ((e.vertexAttribDivisor = function (e, n) {
          t.vertexAttribDivisorANGLE(e, n);
        }),
          (e.drawArraysInstanced = function (e, n, s, o) {
            t.drawArraysInstancedANGLE(e, n, s, o);
          }),
          (e.drawElementsInstanced = function (e, n, s, o, i) {
            t.drawElementsInstancedANGLE(e, n, s, o, i);
          }));
    }
    function s(e) {
      var t = e.getExtension("EXT_disjoint_timer_query");
      t &&
        ((e.createQuery = function () {
          return t.createQueryEXT();
        }),
          (e.beginQuery = function (e, n) {
            return t.beginQueryEXT(e, n);
          }),
          (e.endQuery = function (e) {
            return t.endQueryEXT(e);
          }),
          (e.deleteQuery = function (e) {
            t.deleteQueryEXT(e);
          }),
          (e.getQueryObject = function (e, n) {
            return t.getQueryObjectEXT(e, n);
          }));
    }
    function o(e) {
      var t = e.getExtension("WEBGL_draw_buffers");
      t &&
        (e.drawBuffers = function (e) {
          return t.drawBuffersWEBGL(e);
        });
    }
    try {
      gl.getExtension("EXT_shader_texture_lod"), gl.getExtension("OES_standard_derivatives");
    } catch (e) {
      console.warn(e);
    }
    t(gl), n(gl), s(gl), o(gl), gl.getExtension("WEBGL_depth_texture") == null && alert("Cant initialize WEBGL_depth_texture extension");
  } else gl = canvas.getContext("webgl2");
  gl === null && alert("Unable to initialize WebGL. Your browser or machine may not support it.");
}
canvas.focus(), (canvas.requestPointerLock = canvas.requestPointerLock || canvas.mozRequestPointerLock || function () { }), (document.exitPointerLock = document.exitPointerLock || document.mozExitPointerLock || function () { });
function assert(e, t) {
  e == !1 && alert(t);
}
function getArray(e, t, n) {
  return new t(wasm_memory.buffer, e, n);
}
function UTF8ToString(e, t) {
  let i = new Uint8Array(wasm_memory.buffer, e);
  for (var n, a, r, c, s = 0, l = s + t, o = ""; !(s >= l);) {
    if (((n = i[s++]), !n)) return o;
    if (!(n & 128)) {
      o += String.fromCharCode(n);
      continue;
    }
    if (((a = i[s++] & 63), (n & 224) == 192)) {
      o += String.fromCharCode(((n & 31) << 6) | a);
      continue;
    }
    (r = i[s++] & 63),
      (n & 240) == 224
        ? (n = ((n & 15) << 12) | (a << 6) | r)
        : ((n & 248) != 240 && console.warn("Invalid UTF-8 leading byte 0x" + n.toString(16) + " encountered when deserializing a UTF-8 string on the asm.js/wasm heap to a JS string!"),
          (n = ((n & 7) << 18) | (a << 12) | (r << 6) | (i[s++] & 63))),
      n < 65536 ? (o += String.fromCharCode(n)) : ((c = n - 65536), (o += String.fromCharCode(55296 | (c >> 10), 56320 | (c & 1023))));
  }
  return o;
}
function stringToUTF8(e, t, n, s) {
  for (var o, r, c = n, i = n + s, a = 0; a < e.length; ++a)
    if (((o = e.charCodeAt(a)), o >= 55296 && o <= 57343 && ((r = e.charCodeAt(++a)), (o = (65536 + ((o & 1023) << 10)) | (r & 1023))), o <= 127)) {
      if (n >= i) break;
      t[n++] = o;
    } else if (o <= 2047) {
      if (n + 1 >= i) break;
      (t[n++] = 192 | (o >> 6)), (t[n++] = 128 | (o & 63));
    } else if (o <= 65535) {
      if (n + 2 >= i) break;
      (t[n++] = 224 | (o >> 12)), (t[n++] = 128 | ((o >> 6) & 63)), (t[n++] = 128 | (o & 63));
    } else {
      if (n + 3 >= i) break;
      o >= 2097152 && console.warn("Invalid Unicode code point 0x" + o.toString(16) + " encountered when serializing a JS string to an UTF-8 string on the asm.js/wasm heap! (Valid unicode code points should be in range 0-0x1FFFFF)."),
        (t[n++] = 240 | (o >> 18)),
        (t[n++] = 128 | ((o >> 12) & 63)),
        (t[n++] = 128 | ((o >> 6) & 63)),
        (t[n++] = 128 | (o & 63));
    }
  return n - c;
}
(FS = { loaded_files: [], unique_id: 0 }),
  (GL = {
    counter: 1,
    buffers: [],
    mappedBuffers: {},
    programs: [],
    framebuffers: [],
    renderbuffers: [],
    textures: [],
    uniforms: [],
    shaders: [],
    vaos: [],
    timerQueries: [],
    contexts: {},
    programInfos: {},
    getNewId: function (e) {
      for (var n = GL.counter++, t = e.length; t < n; t++) e[t] = null;
      return n;
    },
    validateGLObjectID: function (e, t, n, s) {
      t != 0 && (e[t] === null ? console.error(n + " called with an already deleted " + s + " ID " + t + "!") : e[t] || console.error(n + " called with an invalid " + s + " ID " + t + "!"));
    },
    getSource: function (e, t, n, s) {
      for (var a, i = "", o = 0; o < t; ++o) (a = s == 0 ? void 0 : getArray(s + o * 4, Uint32Array, 1)[0]), (i += UTF8ToString(getArray(n + o * 4, Uint32Array, 1)[0], a));
      return i;
    },
    populateUniformTable: function (e) {
      GL.validateGLObjectID(GL.programs, e, "populateUniformTable", "program");
      for (
        var t, n, s, i, a, l, o = GL.programs[e], r = (GL.programInfos[e] = { uniforms: {}, maxUniformLength: 0, maxAttributeLength: -1, maxUniformBlockNameLength: -1 }), d = r.uniforms, u = gl.getProgramParameter(o, 35718), c = 0;
        c < u;
        ++c
      )
        if (((i = gl.getActiveUniform(o, c)), (t = i.name), (r.maxUniformLength = Math.max(r.maxUniformLength, t.length + 1)), t.slice(-1) == "]" && (t = t.slice(0, t.lastIndexOf("["))), (n = gl.getUniformLocation(o, t)), n)) {
          (s = GL.getNewId(GL.uniforms)), (d[t] = [i.size, s]), (GL.uniforms[s] = n);
          for (a = 1; a < i.size; ++a) (l = t + "[" + a + "]"), (n = gl.getUniformLocation(o, l)), (s = GL.getNewId(GL.uniforms)), (GL.uniforms[s] = n);
        }
    },
  });
function _glGenObject(e, t, n, s, o) {
  for (var i, a, r = 0; r < e; r++)
    (i = gl[n]()),
      (a = i && GL.getNewId(s)),
      i ? ((i.name = a), (s[a] = i)) : (console.error("GL_INVALID_OPERATION"), GL.recordError(1282), alert("GL_INVALID_OPERATION in " + o + ": GLctx." + n + " returned null - most likely GL context is lost!")),
      (getArray(t + r * 4, Int32Array, 1)[0] = a);
}
function _webglGet(e, t, n) {
  if (!t) {
    console.error("GL_INVALID_VALUE in glGet" + n + "v(name=" + e + ": Function called with null out pointer!"), GL.recordError(1281);
    return;
  }
  var s,
    i,
    a,
    o = void 0;
  switch (e) {
    case 36346:
      o = 1;
      break;
    case 36344:
      n != "EM_FUNC_SIG_PARAM_I" && n != "EM_FUNC_SIG_PARAM_I64" && (GL.recordError(1280), err("GL_INVALID_ENUM in glGet" + n + "v(GL_SHADER_BINARY_FORMATS): Invalid parameter type!"));
      return;
    case 34814:
    case 36345:
      o = 0;
      break;
    case 34466:
      (i = gl.getParameter(34467)), (o = i ? i.length : 0);
      break;
    case 33309:
      assert(!1, "unimplemented");
      break;
    case 33307:
    case 33308:
      assert(!1, "unimplemented");
      break;
  }
  if (o === void 0)
    switch (((s = gl.getParameter(e)), typeof s)) {
      case "number":
        o = s;
        break;
      case "boolean":
        o = s ? 1 : 0;
        break;
      case "string":
        GL.recordError(1280), console.error("GL_INVALID_ENUM in glGet" + n + "v(" + e + ") on a name which returns a string!");
        return;
      case "object":
        if (s === null)
          switch (e) {
            case 34964:
            case 35725:
            case 34965:
            case 36006:
            case 36007:
            case 32873:
            case 34229:
            case 35097:
            case 36389:
            case 34068: {
              o = 0;
              break;
            }
            default: {
              GL.recordError(1280), console.error("GL_INVALID_ENUM in glGet" + n + "v(" + e + ") and it returns null!");
              return;
            }
          }
        else if (s instanceof Float32Array || s instanceof Uint32Array || s instanceof Int32Array || s instanceof Array) {
          for (a = 0; a < s.length; ++a) assert(!1, "unimplemented");
          return;
        } else
          try {
            o = s.name | 0;
          } catch (t) {
            GL.recordError(1280), console.error("GL_INVALID_ENUM in glGet" + n + "v: Unknown object returned from WebGL getParameter(" + e + ")! (error: " + t + ")");
            return;
          }
        break;
      default:
        GL.recordError(1280), console.error("GL_INVALID_ENUM in glGet" + n + "v: Native code calling glGet" + n + "v(" + e + ") and it returns " + s + " of type " + typeof s + "!");
        return;
    }
  switch (n) {
    case "EM_FUNC_SIG_PARAM_I64":
      getArray(t, Int32Array, 1)[0] = o;
    case "EM_FUNC_SIG_PARAM_I":
      getArray(t, Int32Array, 1)[0] = o;
      break;
    case "EM_FUNC_SIG_PARAM_F":
      getArray(t, Float32Array, 1)[0] = o;
      break;
    case "EM_FUNC_SIG_PARAM_B":
      getArray(t, Int8Array, 1)[0] = o ? 1 : 0;
      break;
    default:
      throw "internal glGet error, bad type: " + n;
  }
}
function resize(e, t) {
  var o = dpi_scale(),
    n = e.clientWidth * o,
    s = e.clientHeight * o;
  (e.width != n || e.height != s) && ((e.width = n), (e.height = s), t != null && t(Math.floor(n), Math.floor(s)));
}
function animation() {
  wasm_exports.frame(), window.blocking_event_loop || (animation_frame_timeout && window.cancelAnimationFrame(animation_frame_timeout), (animation_frame_timeout = window.requestAnimationFrame(animation)));
}
const SAPP_EVENTTYPE_TOUCHES_BEGAN = 10,
  SAPP_EVENTTYPE_TOUCHES_MOVED = 11,
  SAPP_EVENTTYPE_TOUCHES_ENDED = 12,
  SAPP_EVENTTYPE_TOUCHES_CANCELED = 13,
  SAPP_MODIFIER_SHIFT = 1,
  SAPP_MODIFIER_CTRL = 2,
  SAPP_MODIFIER_ALT = 4,
  SAPP_MODIFIER_SUPER = 8;
function into_sapp_mousebutton(e) {
  switch (e) {
    case 0:
      return 0;
    case 1:
      return 2;
    case 2:
      return 1;
    default:
      return e;
  }
}
function into_sapp_keycode(e) {
  switch (e) {
    // Movement keys
    case "KeyW":
      return 87;
    case "KeyA":
      return 65;
    case "KeyS":
      return 83;
    case "KeyD":
      return 68;
    // Arrow keys
    case "ArrowRight":
      return 262;
    case "ArrowLeft":
      return 263;
    case "ArrowDown":
      return 264;
    case "ArrowUp":
      return 265;
  }
  console.log("Unsupported keyboard key: ", e);
}
function dpi_scale() {
  return high_dpi ? window.devicePixelRatio || 1 : 1;
}
function texture_size(e, t, n) {
  return e == gl.ALPHA ? t * n : e == gl.RGB ? t * n * 3 : e == gl.RGBA ? t * n * 4 : t * n * 3;
}
function mouse_relative_position(e, t) {
  var n = canvas.getBoundingClientRect(),
    s = (e - n.left) * dpi_scale(),
    o = (t - n.top) * dpi_scale();
  return { x: s, y: o };
}
(emscripten_shaders_hack = !1),
  (importObject = {
    env: {
      console_debug: function (e) {
        console.debug(UTF8ToString(e));
      },
      console_log: function (e) {
        console.log(UTF8ToString(e));
      },
      console_info: function (e) {
        console.info(UTF8ToString(e));
      },
      console_warn: function (e) {
        console.warn(UTF8ToString(e));
      },
      console_error: function (e) {
        console.error(UTF8ToString(e));
      },
      set_emscripten_shader_hack: function (e) {
        emscripten_shaders_hack = e;
      },
      sapp_set_clipboard: function (e, t) {
        clipboard = UTF8ToString(e, t);
      },
      dpi_scale,
      rand: function () {
        return Math.floor(Math.random() * 2147483647);
      },
      now: function () {
        return Date.now() / 1e3;
      },
      canvas_width: function () {
        return Math.floor(canvas.width);
      },
      canvas_height: function () {
        return Math.floor(canvas.height);
      },
      glClearDepthf: function (e) {
        gl.clearDepth(e);
      },
      glClearColor: function (e, t, n, s) {
        gl.clearColor(e, t, n, s);
      },
      glClearStencil: function (e) {
        gl.clearStencil(e);
      },
      glColorMask: function (e, t, n, s) {
        gl.colorMask(e, t, n, s);
      },
      glScissor: function (e, t, n, s) {
        gl.scissor(e, t, n, s);
      },
      glClear: function (e) {
        gl.clear(e);
      },
      glGenTextures: function (e, t) {
        _glGenObject(e, t, "createTexture", GL.textures, "glGenTextures");
      },
      glActiveTexture: function (e) {
        gl.activeTexture(e);
      },
      glBindTexture: function (e, t) {
        GL.validateGLObjectID(GL.textures, t, "glBindTexture", "texture"), gl.bindTexture(e, GL.textures[t]);
      },
      glTexImage2D: function (e, t, n, s, o, i, a, r, c) {
        gl.texImage2D(e, t, n, s, o, i, a, r, c ? getArray(c, Uint8Array, texture_size(n, s, o)) : null);
      },
      glTexSubImage2D: function (e, t, n, s, o, i, a, r, c) {
        gl.texSubImage2D(e, t, n, s, o, i, a, r, c ? getArray(c, Uint8Array, texture_size(a, o, i)) : null);
      },
      glReadPixels: function (e, t, n, s, o, i, a) {
        var r = getArray(a, Uint8Array, texture_size(o, n, s));
        gl.readPixels(e, t, n, s, o, i, r);
      },
      glTexParameteri: function (e, t, n) {
        gl.texParameteri(e, t, n);
      },
      glUniform1fv: function (e, t, n) {
        GL.validateGLObjectID(GL.uniforms, e, "glUniform1fv", "location"), assert((n & 3) == 0, "Pointer to float data passed to glUniform1fv must be aligned to four bytes!");
        var s = getArray(n, Float32Array, 1 * t);
        gl.uniform1fv(GL.uniforms[e], s);
      },
      glUniform2fv: function (e, t, n) {
        GL.validateGLObjectID(GL.uniforms, e, "glUniform2fv", "location"), assert((n & 3) == 0, "Pointer to float data passed to glUniform2fv must be aligned to four bytes!");
        var s = getArray(n, Float32Array, 2 * t);
        gl.uniform2fv(GL.uniforms[e], s);
      },
      glUniform3fv: function (e, t, n) {
        GL.validateGLObjectID(GL.uniforms, e, "glUniform3fv", "location"), assert((n & 3) == 0, "Pointer to float data passed to glUniform3fv must be aligned to four bytes!");
        var s = getArray(n, Float32Array, 3 * t);
        gl.uniform3fv(GL.uniforms[e], s);
      },
      glUniform4fv: function (e, t, n) {
        GL.validateGLObjectID(GL.uniforms, e, "glUniform4fv", "location"), assert((n & 3) == 0, "Pointer to float data passed to glUniform4fv must be aligned to four bytes!");
        var s = getArray(n, Float32Array, 4 * t);
        gl.uniform4fv(GL.uniforms[e], s);
      },
      glUniform1iv: function (e, t, n) {
        GL.validateGLObjectID(GL.uniforms, e, "glUniform1fv", "location"), assert((n & 3) == 0, "Pointer to i32 data passed to glUniform1iv must be aligned to four bytes!");
        var s = getArray(n, Int32Array, 1 * t);
        gl.uniform1iv(GL.uniforms[e], s);
      },
      glUniform2iv: function (e, t, n) {
        GL.validateGLObjectID(GL.uniforms, e, "glUniform2fv", "location"), assert((n & 3) == 0, "Pointer to i32 data passed to glUniform2iv must be aligned to four bytes!");
        var s = getArray(n, Int32Array, 2 * t);
        gl.uniform2iv(GL.uniforms[e], s);
      },
      glUniform3iv: function (e, t, n) {
        GL.validateGLObjectID(GL.uniforms, e, "glUniform3fv", "location"), assert((n & 3) == 0, "Pointer to i32 data passed to glUniform3iv must be aligned to four bytes!");
        var s = getArray(n, Int32Array, 3 * t);
        gl.uniform3iv(GL.uniforms[e], s);
      },
      glUniform4iv: function (e, t, n) {
        GL.validateGLObjectID(GL.uniforms, e, "glUniform4fv", "location"), assert((n & 3) == 0, "Pointer to i32 data passed to glUniform4iv must be aligned to four bytes!");
        var s = getArray(n, Int32Array, 4 * t);
        gl.uniform4iv(GL.uniforms[e], s);
      },
      glBlendFunc: function (e, t) {
        gl.blendFunc(e, t);
      },
      glBlendEquationSeparate: function (e, t) {
        gl.blendEquationSeparate(e, t);
      },
      glDisable: function (e) {
        gl.disable(e);
      },
      glDrawElements: function (e, t, n, s) {
        gl.drawElements(e, t, n, s);
      },
      glGetIntegerv: function (e, t) {
        _webglGet(e, t, "EM_FUNC_SIG_PARAM_I");
      },
      glUniform1f: function (e, t) {
        GL.validateGLObjectID(GL.uniforms, e, "glUniform1f", "location"), gl.uniform1f(GL.uniforms[e], t);
      },
      glUniform1i: function (e, t) {
        GL.validateGLObjectID(GL.uniforms, e, "glUniform1i", "location"), gl.uniform1i(GL.uniforms[e], t);
      },
      glGetAttribLocation: function (e, t) {
        return gl.getAttribLocation(GL.programs[e], UTF8ToString(t));
      },
      glEnableVertexAttribArray: function (e) {
        gl.enableVertexAttribArray(e);
      },
      glDisableVertexAttribArray: function (e) {
        gl.disableVertexAttribArray(e);
      },
      glVertexAttribPointer: function (e, t, n, s, o, i) {
        gl.vertexAttribPointer(e, t, n, !!s, o, i);
      },
      glVertexAttribIPointer: function (e, t, n, s, o) {
        gl.vertexAttribIPointer(e, t, n, s, o);
      },
      glGetUniformLocation: function (e, t) {
        GL.validateGLObjectID(GL.programs, e, "glGetUniformLocation", "program"), (t = UTF8ToString(t));
        var s,
          o,
          n = 0;
        return (
          t[t.length - 1] == "]" && ((s = t.lastIndexOf("[")), (n = t[s + 1] != "]" ? parseInt(t.slice(s + 1)) : 0), (t = t.slice(0, s))),
          (o = GL.programInfos[e] && GL.programInfos[e].uniforms[t]),
          o && n >= 0 && n < o[0] ? o[1] + n : -1
        );
      },
      glUniformMatrix4fv: function (e, t, n, s) {
        GL.validateGLObjectID(GL.uniforms, e, "glUniformMatrix4fv", "location"), assert((s & 3) == 0, "Pointer to float data passed to glUniformMatrix4fv must be aligned to four bytes!");
        var o = getArray(s, Float32Array, 16);
        gl.uniformMatrix4fv(GL.uniforms[e], !!n, o);
      },
      glUseProgram: function (e) {
        GL.validateGLObjectID(GL.programs, e, "glUseProgram", "program"), gl.useProgram(GL.programs[e]);
      },
      glGenVertexArrays: function (e, t) {
        _glGenObject(e, t, "createVertexArray", GL.vaos, "glGenVertexArrays");
      },
      glGenFramebuffers: function (e, t) {
        _glGenObject(e, t, "createFramebuffer", GL.framebuffers, "glGenFramebuffers");
      },
      glGenRenderbuffers: function (e, t) {
        _glGenObject(e, t, "createRenderbuffer", GL.renderbuffers, "glGenRenderbuffers");
      },
      glBindVertexArray: function (e) {
        gl.bindVertexArray(GL.vaos[e]);
      },
      glBindFramebuffer: function (e, t) {
        GL.validateGLObjectID(GL.framebuffers, t, "glBindFramebuffer", "framebuffer"), gl.bindFramebuffer(e, GL.framebuffers[t]);
      },
      glBindRenderbuffer: function (e, t) {
        GL.validateGLObjectID(GL.renderbuffers, t, "glBindRenderbuffer", "renderbuffer"), gl.bindRenderbuffer(e, GL.renderbuffers[t]);
      },
      glGenBuffers: function (e, t) {
        _glGenObject(e, t, "createBuffer", GL.buffers, "glGenBuffers");
      },
      glBindBuffer: function (e, t) {
        GL.validateGLObjectID(GL.buffers, t, "glBindBuffer", "buffer"), gl.bindBuffer(e, GL.buffers[t]);
      },
      glBufferData: function (e, t, n, s) {
        gl.bufferData(e, n ? getArray(n, Uint8Array, t) : t, s);
      },
      glBufferSubData: function (e, t, n, s) {
        gl.bufferSubData(e, t, s ? getArray(s, Uint8Array, n) : n);
      },
      glEnable: function (e) {
        gl.enable(e);
      },
      glFlush: function () {
        gl.flush();
      },
      glFinish: function () {
        gl.finish();
      },
      glDepthFunc: function (e) {
        gl.depthFunc(e);
      },
      glBlendFuncSeparate: function (e, t, n, s) {
        gl.blendFuncSeparate(e, t, n, s);
      },
      glViewport: function (e, t, n, s) {
        gl.viewport(e, t, n, s);
      },
      glDrawArrays: function (e, t, n) {
        gl.drawArrays(e, t, n);
      },
      glDrawBuffers: function (e, t) {
        gl.drawBuffers(getArray(t, Int32Array, e));
      },
      glCreateProgram: function () {
        var e = GL.getNewId(GL.programs),
          t = gl.createProgram();
        return (t.name = e), (GL.programs[e] = t), e;
      },
      glAttachShader: function (e, t) {
        GL.validateGLObjectID(GL.programs, e, "glAttachShader", "program"), GL.validateGLObjectID(GL.shaders, t, "glAttachShader", "shader"), gl.attachShader(GL.programs[e], GL.shaders[t]);
      },
      glDetachShader: function (e, t) {
        GL.validateGLObjectID(GL.programs, e, "glDetachShader", "program"), GL.validateGLObjectID(GL.shaders, t, "glDetachShader", "shader"), gl.detachShader(GL.programs[e], GL.shaders[t]);
      },
      glLinkProgram: function (e) {
        GL.validateGLObjectID(GL.programs, e, "glLinkProgram", "program"), gl.linkProgram(GL.programs[e]), GL.populateUniformTable(e);
      },
      glPixelStorei: function (e, t) {
        gl.pixelStorei(e, t);
      },
      glFramebufferTexture2D: function (e, t, n, s, o) {
        GL.validateGLObjectID(GL.textures, s, "glFramebufferTexture2D", "texture"), gl.framebufferTexture2D(e, t, n, GL.textures[s], o);
      },
      glGetProgramiv: function (e, t, n) {
        if ((assert(n), GL.validateGLObjectID(GL.programs, e, "glGetProgramiv", "program"), e >= GL.counter)) {
          console.error("GL_INVALID_VALUE in glGetProgramiv");
          return;
        }
        var s,
          o = GL.programInfos[e];
        if (!o) {
          console.error("GL_INVALID_OPERATION in glGetProgramiv(program=" + e + ", pname=" + t + ", p=0x" + n.toString(16) + "): The specified GL object name does not refer to a program object!");
          return;
        }
        if (t == 35716) (s = gl.getProgramInfoLog(GL.programs[e])), assert(s !== null), (getArray(n, Int32Array, 1)[0] = s.length + 1);
        else if (t == 35719) {
          console.error("unsupported operation");
          return;
        } else if (t == 35722) {
          console.error("unsupported operation");
          return;
        } else if (t == 35381) {
          console.error("unsupported operation");
          return;
        } else getArray(n, Int32Array, 1)[0] = gl.getProgramParameter(GL.programs[e], t);
      },
      glCreateShader: function (e) {
        var t = GL.getNewId(GL.shaders);
        return (GL.shaders[t] = gl.createShader(e)), t;
      },
      glStencilFuncSeparate: function (e, t, n, s) {
        gl.stencilFuncSeparate(e, t, n, s);
      },
      glStencilMaskSeparate: function (e, t) {
        gl.stencilMaskSeparate(e, t);
      },
      glStencilOpSeparate: function (e, t, n, s) {
        gl.stencilOpSeparate(e, t, n, s);
      },
      glFrontFace: function (e) {
        gl.frontFace(e);
      },
      glCullFace: function (e) {
        gl.cullFace(e);
      },
      glCopyTexImage2D: function (e, t, n, s, o, i, a, r) {
        gl.copyTexImage2D(e, t, n, s, o, i, a, r);
      },
      glShaderSource: function (e, t, n, s) {
        GL.validateGLObjectID(GL.shaders, e, "glShaderSource", "shader");
        var i,
          o = GL.getSource(e, t, n, s);
        emscripten_shaders_hack &&
          ((o = o.replace(/#extension GL_OES_standard_derivatives : enable/g, "")),
            (o = o.replace(/#extension GL_EXT_shader_texture_lod : enable/g, "")),
            (i = ""),
            o.indexOf("gl_FragColor") != -1 &&
            ((i += `out mediump vec4 GL_FragColor;
`),
              (o = o.replace(/gl_FragColor/g, "GL_FragColor"))),
            o.indexOf("attribute") != -1 ? ((o = o.replace(/attribute/g, "in")), (o = o.replace(/varying/g, "out"))) : (o = o.replace(/varying/g, "in")),
            (o = o.replace(/textureCubeLodEXT/g, "textureCubeLod")),
            (o = o.replace(/texture2DLodEXT/g, "texture2DLod")),
            (o = o.replace(/texture2DProjLodEXT/g, "texture2DProjLod")),
            (o = o.replace(/texture2DGradEXT/g, "texture2DGrad")),
            (o = o.replace(/texture2DProjGradEXT/g, "texture2DProjGrad")),
            (o = o.replace(/textureCubeGradEXT/g, "textureCubeGrad")),
            (o = o.replace(/textureCube/g, "texture")),
            (o = o.replace(/texture1D/g, "texture")),
            (o = o.replace(/texture2D/g, "texture")),
            (o = o.replace(/texture3D/g, "texture")),
            (o = o.replace(
              /#version 100/g,
              `#version 300 es
` + i
            ))),
          gl.shaderSource(GL.shaders[e], o);
      },
      glGetProgramInfoLog: function (e, t, n, s) {
        GL.validateGLObjectID(GL.programs, e, "glGetProgramInfoLog", "program");
        var o,
          i = gl.getProgramInfoLog(GL.programs[e]);
        assert(i !== null);
        let a = getArray(s, Uint8Array, t);
        for (o = 0; o < t; o++) a[o] = i.charCodeAt(o);
      },
      glGetString: function (e) {
        var t = gl.getParameter(e).toString(),
          n = t.length + 1,
          s = wasm_exports.allocate_vec_u8(n),
          o = new Uint8Array(wasm_memory.buffer, s, n);
        return (o[t.length] = 0), stringToUTF8(t, o, 0, n), s;
      },
      glCompileShader: function (e) {
        GL.validateGLObjectID(GL.shaders, e, "glCompileShader", "shader"), gl.compileShader(GL.shaders[e]);
      },
      glGetShaderiv: function (e, t, n) {
        if ((assert(n), GL.validateGLObjectID(GL.shaders, e, "glGetShaderiv", "shader"), t == 35716)) {
          var s,
            i,
            o = gl.getShaderInfoLog(GL.shaders[e]);
          assert(o !== null), (getArray(n, Int32Array, 1)[0] = o.length + 1);
        } else
          t == 35720
            ? ((s = gl.getShaderSource(GL.shaders[e])), (i = s === null || s.length == 0 ? 0 : s.length + 1), (getArray(n, Int32Array, 1)[0] = i))
            : (getArray(n, Int32Array, 1)[0] = gl.getShaderParameter(GL.shaders[e], t));
      },
      glGetShaderInfoLog: function (e, t, n, s) {
        GL.validateGLObjectID(GL.shaders, e, "glGetShaderInfoLog", "shader");
        var o,
          i = gl.getShaderInfoLog(GL.shaders[e]);
        assert(i !== null);
        let a = getArray(s, Uint8Array, t);
        for (o = 0; o < t; o++) a[o] = i.charCodeAt(o);
      },
      glVertexAttribDivisor: function (e, t) {
        gl.vertexAttribDivisor(e, t);
      },
      glDrawArraysInstanced: function (e, t, n, s) {
        gl.drawArraysInstanced(e, t, n, s);
      },
      glDrawElementsInstanced: function (e, t, n, s, o) {
        gl.drawElementsInstanced(e, t, n, s, o);
      },
      glDeleteShader: function (e) {
        var t = GL.shaders[e];
        if (t == null) return;
        gl.deleteShader(t), (GL.shaders[e] = null);
      },
      glDeleteProgram: function (e) {
        var t = GL.programs[e];
        if (t == null) return;
        gl.deleteProgram(t), (GL.programs[e] = null);
      },
      glDeleteBuffers: function (e, t) {
        for (var n, o, s = 0; s < e; s++) {
          if (((o = getArray(t + s * 4, Uint32Array, 1)[0]), (n = GL.buffers[o]), !n)) continue;
          gl.deleteBuffer(n), (n.name = 0), (GL.buffers[o] = null);
        }
      },
      glDeleteFramebuffers: function (e, t) {
        for (var n, o, s = 0; s < e; s++) {
          if (((o = getArray(t + s * 4, Uint32Array, 1)[0]), (n = GL.framebuffers[o]), !n)) continue;
          gl.deleteFramebuffer(n), (n.name = 0), (GL.framebuffers[o] = null);
        }
      },
      glDeleteTextures: function (e, t) {
        for (var n, o, s = 0; s < e; s++) {
          if (((o = getArray(t + s * 4, Uint32Array, 1)[0]), (n = GL.textures[o]), !n)) continue;
          gl.deleteTexture(n), (n.name = 0), (GL.textures[o] = null);
        }
      },
      glGenQueries: function (e, t) {
        _glGenObject(e, t, "createQuery", GL.timerQueries, "glGenQueries");
      },
      glDeleteQueries: function (e) {
        for (var n, o, s = 0; s < e; s++) {
          if (((o = getArray(textures + s * 4, Uint32Array, 1)[0]), (n = GL.timerQueries[o]), !n)) continue;
          gl.deleteQuery(n), (n.name = 0), (GL.timerQueries[o] = null);
        }
      },
      glBeginQuery: function (e, t) {
        GL.validateGLObjectID(GL.timerQueries, t, "glBeginQuery", "id"), gl.beginQuery(e, GL.timerQueries[t]);
      },
      glEndQuery: function (e) {
        gl.endQuery(e);
      },
      glGetQueryObjectiv: function (e, t, n) {
        GL.validateGLObjectID(GL.timerQueries, e, "glGetQueryObjectiv", "id");
        let s = gl.getQueryObject(GL.timerQueries[e], t);
        getArray(n, Uint32Array, 1)[0] = s;
      },
      glGetQueryObjectui64v: function (e, t, n) {
        GL.validateGLObjectID(GL.timerQueries, e, "glGetQueryObjectui64v", "id");
        let o = gl.getQueryObject(GL.timerQueries[e], t),
          s = getArray(n, Uint32Array, 2);
        (s[0] = o), (s[1] = (o - s[0]) / 4294967296);
      },
      glGenerateMipmap: function (e) {
        gl.generateMipmap(e);
      },
      setup_canvas_size: function (e) {
        (window.high_dpi = e), resize(canvas);
      },
      run_animation_loop: function (e) {
        (canvas.onmousemove = function (e) {
          var t = mouse_relative_position(e.clientX, e.clientY),
            n = t.x,
            s = t.y;
          wasm_exports.mouse_move(Math.floor(n), Math.floor(s)), (e.movementX != 0 || e.movementY != 0) && wasm_exports.raw_mouse_move(Math.floor(e.movementX), Math.floor(e.movementY));
        }),
          (canvas.onmousedown = function (e) {
            var t = mouse_relative_position(e.clientX, e.clientY),
              n = t.x,
              s = t.y,
              o = into_sapp_mousebutton(e.button);
            wasm_exports.mouse_down(n, s, o);
          }),
          canvas.addEventListener("wheel", function (e) {
            e.preventDefault(), wasm_exports.mouse_wheel(-e.deltaX, -e.deltaY);
          }),
          (canvas.onmouseup = function (e) {
            var t = mouse_relative_position(e.clientX, e.clientY),
              n = t.x,
              s = t.y,
              o = into_sapp_mousebutton(e.button);
            wasm_exports.mouse_up(n, s, o);
          }),
          (canvas.onkeydown = function (e) {
            var n,
              t = into_sapp_keycode(e.code);
            switch (t) {
              case 32:
              case 262:
              case 263:
              case 264:
              case 265:
              case 290:
              case 291:
              case 292:
              case 293:
              case 294:
              case 295:
              case 296:
              case 297:
              case 298:
              case 299:
              case 259:
              case 258:
              case 39:
              case 47:
                e.preventDefault();
                break;
            }
            (n = 0),
              e.ctrlKey && (n |= SAPP_MODIFIER_CTRL),
              e.shiftKey && (n |= SAPP_MODIFIER_SHIFT),
              e.altKey && (n |= SAPP_MODIFIER_ALT),
              wasm_exports.key_down(t, n, e.repeat),
              (t == 32 || t == 39 || t == 47) && wasm_exports.key_press(t);
          }),
          (canvas.onkeyup = function (e) {
            var n = into_sapp_keycode(e.code),
              t = 0;
            e.ctrlKey && (t |= SAPP_MODIFIER_CTRL), e.shiftKey && (t |= SAPP_MODIFIER_SHIFT), e.altKey && (t |= SAPP_MODIFIER_ALT), wasm_exports.key_up(n, t);
          }),
          (canvas.onkeypress = function (e) {
            var t = into_sapp_keycode(e.code);
            let n = t == 261 || e.ctrlKey;
            n == !1 && wasm_exports.key_press(e.charCode);
          }),
          canvas.addEventListener("touchstart", function (e) {
            e.preventDefault();
            for (const touch of e.changedTouches) {
              let pos = mouse_relative_position(touch.clientX, touch.clientY);
              wasm_exports.touch(SAPP_EVENTTYPE_TOUCHES_BEGAN, touch.identifier, pos.x, pos.y);
            }
          }, { passive: false }),
          canvas.addEventListener("touchend", function (e) {
            e.preventDefault();
            for (const touch of e.changedTouches) {
              let pos = mouse_relative_position(touch.clientX, touch.clientY);
              wasm_exports.touch(SAPP_EVENTTYPE_TOUCHES_ENDED, touch.identifier, pos.x, pos.y);
            }
          }, { passive: false }),
          canvas.addEventListener("touchcancel", function (e) {
            e.preventDefault();
            for (const touch of e.changedTouches) {
              let pos = mouse_relative_position(touch.clientX, touch.clientY);
              wasm_exports.touch(SAPP_EVENTTYPE_TOUCHES_CANCELED, touch.identifier, pos.x, pos.y);
            }
          }, { passive: false }),
          canvas.addEventListener("touchmove", function (e) {
            e.preventDefault();
            for (const touch of e.changedTouches) {
              let pos = mouse_relative_position(touch.clientX, touch.clientY);
              wasm_exports.touch(SAPP_EVENTTYPE_TOUCHES_MOVED, touch.identifier, pos.x, pos.y);
            }
          }, { passive: false }),
          (window.onresize = function () {
            resize(canvas, wasm_exports.resize);
          }),
          (window.ondrop = async function (e) {
            e.preventDefault(), wasm_exports.on_files_dropped_start();
            for (let n of e.dataTransfer.files) {
              const t = n.name.length,
                o = wasm_exports.allocate_vec_u8(t),
                r = new Uint8Array(wasm_memory.buffer, o, t);
              stringToUTF8(n.name, r, 0, t);
              const i = await n.arrayBuffer(),
                s = i.byteLength,
                a = wasm_exports.allocate_vec_u8(s),
                c = new Uint8Array(wasm_memory.buffer, a, s);
              c.set(new Uint8Array(i), 0), wasm_exports.on_file_dropped(o, t, a, s);
            }
            wasm_exports.on_files_dropped_finish();
          });
        let n = document.hasFocus();
        var t = function () {
          let e = document.hasFocus();
          n == e && (wasm_exports.focus(e), (n = e));
        };
        document.addEventListener("visibilitychange", t), window.addEventListener("focus", t), window.addEventListener("blur", t), (window.blocking_event_loop = e), window.requestAnimationFrame(animation);
      },
      fs_load_file: function (e, t) {
        var s,
          o = UTF8ToString(e, t),
          n = FS.unique_id;
        return (
          (FS.unique_id += 1),
          (s = new XMLHttpRequest()),
          s.open("GET", o, !0),
          (s.responseType = "arraybuffer"),
          (s.onreadystatechange = function () {
            if (this.readyState === 4)
              if (this.status === 200) {
                var e = new Uint8Array(this.response);
                (FS.loaded_files[n] = e), wasm_exports.file_loaded(n);
              } else (FS.loaded_files[n] = null), wasm_exports.file_loaded(n);
          }),
          s.send(),
          n
        );
      },
      fs_get_buffer_size: function (e) {
        return FS.loaded_files[e] == null ? -1 : FS.loaded_files[e].length;
      },
      fs_take_buffer: function (e, t, n) {
        var s,
          i,
          o = FS.loaded_files[e];
        console.assert(o.length <= n), (i = new Uint8Array(wasm_memory.buffer, t, n));
        for (s = 0; s < o.length; s++) i[s] = o[s];
        delete FS.loaded_files[e];
      },
      sapp_set_cursor_grab: function (e) {
        e ? canvas.requestPointerLock() : document.exitPointerLock();
      },
      sapp_set_cursor: function (e, t) {
        canvas.style.cursor = UTF8ToString(e, t);
      },
      sapp_is_fullscreen: function () {
        let e = document.fullscreenElement;
        return e != null && e.id == canvas.id;
      },
      sapp_set_fullscreen: function (e) {
        e ? canvas.requestFullscreen() : document.exitFullscreen();
      },
      sapp_set_window_size: function (e, t) {
        (canvas.width = e), (canvas.height = t), resize(canvas, wasm_exports.resize);
      },
      sapp_schedule_update: function () {
        animation_frame_timeout && window.cancelAnimationFrame(animation_frame_timeout), (animation_frame_timeout = window.requestAnimationFrame(animation));
      },
      init_webgl,
    },
  });
function register_plugins(e) {
  if (e == null) return;
  for (var t = 0; t < e.length; t++) e[t].register_plugin != void 0 && e[t].register_plugin != null && e[t].register_plugin(importObject);
}
function init_plugins(e) {
  if (e == null) return;
  for (var n, s, t = 0; t < e.length; t++)
    e[t].on_init != void 0 && e[t].on_init != null && e[t].on_init(),
      e[t].name == void 0 || e[t].name == null || e[t].version == void 0 || e[t].version == null
        ? (console.warn("Some of the registred plugins do not have name or version"), console.warn("Probably old version of the plugin used"))
        : ((n = e[t].name + "_crate_version"),
          wasm_exports[n] == void 0
            ? console.log("Plugin " + e[t].name + " is present in JS bundle, but is not used in the rust code.")
            : ((s = wasm_exports[n]()), e[t].version != s && console.error("Plugin " + e[t].name + " version mismatchjs version: " + e[t].version + ", crate version: " + s)));
}
function miniquad_add_plugin(e) {
  plugins.push(e);
}
function add_missing_functions_stabs(e) {
  var t = WebAssembly.Module.imports(e);
  for (const e in t)
    importObject.env[t[e].name] == void 0 &&
      (console.warn("No " + t[e].name + " function in gl.js"),
        (importObject.env[t[e].name] = function () {
          console.warn("Missed function: " + t[e].name);
        }));
}
function load(e) {
  var t = fetch(e);
  register_plugins(plugins),
    typeof WebAssembly.compileStreaming == "function"
      ? WebAssembly.compileStreaming(t)
        .then((e) => (add_missing_functions_stabs(e), WebAssembly.instantiate(e, importObject)))
        .then((e) => {
          (wasm_memory = e.exports.memory), (wasm_exports = e.exports);
          var t = wasm_exports.crate_version();
          version != t && console.error("Version mismatch: gl.js version is: " + version + ", miniquad crate version is: " + t), init_plugins(plugins), e.exports.main();
        })
        .catch((e) => {
          console.error(e);
        })
      : t
        .then(function (e) {
          return e.arrayBuffer();
        })
        .then(function (e) {
          return WebAssembly.compile(e);
        })
        .then(function (e) {
          return add_missing_functions_stabs(e), WebAssembly.instantiate(e, importObject);
        })
        .then(function (e) {
          (wasm_memory = e.exports.memory), (wasm_exports = e.exports);
          var t = wasm_exports.crate_version();
          version != t && console.error("Version mismatch: gl.js version is: " + version + ", rust sapp-wasm crate version is: " + t), init_plugins(plugins), e.exports.main();
        })
        .catch((e) => {
          console.error("WASM failed to load, probably incompatible gl.js version"), console.error(e);
        });
}
