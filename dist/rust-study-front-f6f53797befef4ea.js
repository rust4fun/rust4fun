let _=`function`,U=`undefined`,a2=4,$=`string`,V=`utf-8`,X=0,Z=1,T=null,R=128,a3=16,a0=`Object`,Q=Array,W=Error,a1=JSON.stringify,a5=Object,a6=Promise,a4=Reflect,Y=Uint8Array,S=undefined;var x=((b,c,d,e)=>{const f={a:b,b:c,cnt:Z,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=X;try{return e(c,f.b,...b)}finally{if(--f.cnt===X){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var y=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h79a6d9f6b7b8d326(b,c,k(d))});var s=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==T){return `${a}`};if(b==$){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==T){return `Symbol`}else{return `Symbol(${b})`}};if(b==_){const b=a.name;if(typeof b==$&&b.length>X){return `Function(${b})`}else{return `Function`}};if(Q.isArray(a)){const b=a.length;let c=`[`;if(b>X){c+=s(a[X])};for(let d=Z;d<b;d++){c+=`, `+ s(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>Z){d=c[Z]}else{return toString.call(a)};if(d==a0){try{return `Object(`+ a1(a)+ `)`}catch(a){return a0}};if(a instanceof W){return `${a.name}: ${a.message}\n${a.stack}`};return d});var N=((b,c)=>{a=b.exports;P.__wbindgen_wasm_module=c;q=T;z=T;h=T;a.__wbindgen_start();return a});var j=((a,b)=>{a=a>>>X;return g.decode(i().subarray(a,a+ b))});var p=(a=>a===S||a===T);var O=(b=>{if(a!==S)return a;const c=L();M(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return N(d,b)});var f=(a=>{const b=c(a);e(a);return b});var A=(()=>{if(z===T||z.byteLength===X){z=new Uint32Array(a.memory.buffer)};return z});var t=((b,c,d,e)=>{const f={a:b,b:c,cnt:Z,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===X){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=X}}};g.original=f;return g});var L=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==Z){b.a=X;return !0};const c=!1;return c});b.wbg.__wbindgen_string_new=((a,b)=>{const c=j(a,b);return k(c)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return k(b)});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;r()[a/a2+ Z]=p(d)?X:d;r()[a/a2+ X]=!p(d)});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;r()[a/a2+ Z]=p(d)?X:d;r()[a/a2+ X]=!p(d)});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>X});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>X});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;r()[a/a2+ Z]=p(d)?X:d;r()[a/a2+ X]=!p(d)});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>X});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new W();return k(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a2+ Z]=g;r()[b/a2+ X]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(j(b,c))}finally{a.__wbindgen_free(d,e,Z)}});b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=B(b,c).slice();a.__wbindgen_free(b,c*a2,a2);console.error(...d)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==T;return d});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===_;return b});b.wbg.__wbg_fetch_6a2624d7f767e331=(a=>{const b=fetch(c(a));return k(b)});b.wbg.__wbg_respond_8fadc5f5c9d95422=((a,b)=>{c(a).respond(b>>>X)});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===$?e:S;var g=p(f)?X:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/a2+ Z]=h;r()[b/a2+ X]=g});b.wbg.__wbg_close_e9110ca16e2567db=(a=>{c(a).close()});b.wbg.__wbg_enqueue_d71a1a518e21f5c3=((a,b)=>{c(a).enqueue(c(b))});b.wbg.__wbg_byobRequest_08c18cee35def1f4=(a=>{const b=c(a).byobRequest;return p(b)?X:k(b)});b.wbg.__wbg_close_da7e6fb9d9851e5a=(a=>{c(a).close()});b.wbg.__wbg_view_231340b0dd8a2484=(a=>{const b=c(a).view;return p(b)?X:k(b)});b.wbg.__wbg_buffer_4e79326814bdd393=(a=>{const b=c(a).buffer;return k(b)});b.wbg.__wbg_byteOffset_b69b0a07afccce19=(a=>{const b=c(a).byteOffset;return b});b.wbg.__wbg_byteLength_5299848ed3264181=(a=>{const b=c(a).byteLength;return b});b.wbg.__wbg_queueMicrotask_118eeb525d584d9a=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_26a89c14c53809c0=(a=>{const b=c(a).queueMicrotask;return k(b)});b.wbg.__wbg_body_3eb73da919b867a1=(a=>{const b=c(a).body;return p(b)?X:k(b)});b.wbg.__wbg_createElement_1a136faad4101f43=function(){return C(((a,b,d)=>{const e=c(a).createElement(j(b,d));return k(e)}),arguments)};b.wbg.__wbg_createElementNS_d47e0c50fa2904e0=function(){return C(((a,b,d,e,f)=>{const g=c(a).createElementNS(b===X?S:j(b,d),j(e,f));return k(g)}),arguments)};b.wbg.__wbg_createTextNode_dbdd908f92bae1b1=((a,b,d)=>{const e=c(a).createTextNode(j(b,d));return k(e)});b.wbg.__wbg_instanceof_Window_99dc9805eaa2614b=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5257b70811e953c0=(a=>{const b=c(a).document;return p(b)?X:k(b)});b.wbg.__wbg_instanceof_Element_f614cf57d4316979=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_0819c2800784a176=((b,d)=>{const e=c(d).namespaceURI;var f=p(e)?X:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/a2+ Z]=g;r()[b/a2+ X]=f});b.wbg.__wbg_setinnerHTML_99deeacfff0ae4cc=((a,b,d)=>{c(a).innerHTML=j(b,d)});b.wbg.__wbg_outerHTML_69934f9195df65af=((b,d)=>{const e=c(d).outerHTML;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a2+ Z]=g;r()[b/a2+ X]=f});b.wbg.__wbg_removeAttribute_5c264e727b67dbdb=function(){return C(((a,b,d)=>{c(a).removeAttribute(j(b,d))}),arguments)};b.wbg.__wbg_setAttribute_0918ea45d5a1c663=function(){return C(((a,b,d,e,f)=>{c(a).setAttribute(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_fetch_06d656a1b748ac0d=((a,b)=>{const d=c(a).fetch(c(b));return k(d)});b.wbg.__wbg_instanceof_Response_0d25bb8436a9cefe=(a=>{let b;try{b=c(a) instanceof Response}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_url_47f8307501523859=((b,d)=>{const e=c(d).url;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a2+ Z]=g;r()[b/a2+ X]=f});b.wbg.__wbg_status_351700a30c61ba61=(a=>{const b=c(a).status;return b});b.wbg.__wbg_headers_e38c00d713e8888c=(a=>{const b=c(a).headers;return k(b)});b.wbg.__wbg_arrayBuffer_ec4617b29bb0f61c=function(){return C((a=>{const b=c(a).arrayBuffer();return k(b)}),arguments)};b.wbg.__wbg_newwithstrandinit_9fd2fc855c6327eb=function(){return C(((a,b,d)=>{const e=new Request(j(a,b),c(d));return k(e)}),arguments)};b.wbg.__wbg_parentNode_f3957fdd408a62f7=(a=>{const b=c(a).parentNode;return p(b)?X:k(b)});b.wbg.__wbg_parentElement_86a7612dde875ba9=(a=>{const b=c(a).parentElement;return p(b)?X:k(b)});b.wbg.__wbg_childNodes_75d3da5f3a7bb985=(a=>{const b=c(a).childNodes;return k(b)});b.wbg.__wbg_lastChild_8f7b6f3825115eff=(a=>{const b=c(a).lastChild;return p(b)?X:k(b)});b.wbg.__wbg_nextSibling_13e9454ef5323f1a=(a=>{const b=c(a).nextSibling;return p(b)?X:k(b)});b.wbg.__wbg_setnodeValue_8656e865e9b11bbb=((a,b,d)=>{c(a).nodeValue=b===X?S:j(b,d)});b.wbg.__wbg_textContent_efe8338af53ddf62=((b,d)=>{const e=c(d).textContent;var f=p(e)?X:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/a2+ Z]=g;r()[b/a2+ X]=f});b.wbg.__wbg_cloneNode_80501c66ab115588=function(){return C((a=>{const b=c(a).cloneNode();return k(b)}),arguments)};b.wbg.__wbg_insertBefore_882082ef4c5d7766=function(){return C(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_removeChild_14b08321b677677a=function(){return C(((a,b)=>{const d=c(a).removeChild(c(b));return k(d)}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_cb6366cb0956ce29=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_99e27ed8897850f2=(a=>{const b=c(a).host;return k(b)});b.wbg.__wbg_signal_7876560d9d0f914c=(a=>{const b=c(a).signal;return k(b)});b.wbg.__wbg_new_fa36281638875de8=function(){return C((()=>{const a=new AbortController();return k(a)}),arguments)};b.wbg.__wbg_abort_7792bf3f664d7bb3=(a=>{c(a).abort()});b.wbg.__wbg_addEventListener_1b158e9e95e0ab00=function(){return C(((a,b,d,e,f)=>{c(a).addEventListener(j(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_setchecked_3b12f3d602a63e47=((a,b)=>{c(a).checked=b!==X});b.wbg.__wbg_value_c93cb4b4d352228e=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a2+ Z]=g;r()[b/a2+ X]=f});b.wbg.__wbg_setvalue_9bd3f93b3864ddbf=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_bubbles_f0783dc095f8e220=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_191799b8e0ab3254=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_d94a39b8c8f6eed1=(a=>{const b=c(a).composedPath();return k(b)});b.wbg.__wbg_new_a979e9eedc5e81a3=function(){return C((()=>{const a=new Headers();return k(a)}),arguments)};b.wbg.__wbg_append_047382169b61373d=function(){return C(((a,b,d,e,f)=>{c(a).append(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_value_ab23a75318ea828f=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a2+ Z]=g;r()[b/a2+ X]=f});b.wbg.__wbg_setvalue_918a8ae77531a942=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_debug_0207b724052e591d=((a,b,d,e)=>{console.debug(c(a),c(b),c(d),c(e))});b.wbg.__wbg_error_1f4e3e298a7c97f6=(a=>{console.error(c(a))});b.wbg.__wbg_error_8cf137381b3af25f=((a,b,d,e)=>{console.error(c(a),c(b),c(d),c(e))});b.wbg.__wbg_info_eb81e4fcae9ba8f1=((a,b,d,e)=>{console.info(c(a),c(b),c(d),c(e))});b.wbg.__wbg_log_bd0951a507fbf762=((a,b,d,e)=>{console.log(c(a),c(b),c(d),c(e))});b.wbg.__wbg_warn_ea08466617ec5d3a=((a,b,d,e)=>{console.warn(c(a),c(b),c(d),c(e))});b.wbg.__wbg_get_c43534c00f382c8a=((a,b)=>{const d=c(a)[b>>>X];return k(d)});b.wbg.__wbg_length_d99b680fd68bf71b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_5859b6d41c6fe9f7=((a,b)=>{const c=new Function(j(a,b));return k(c)});b.wbg.__wbg_next_1938cf110c9491d4=(a=>{const b=c(a).next;return k(b)});b.wbg.__wbg_next_267398d0e0761bf9=function(){return C((a=>{const b=c(a).next();return k(b)}),arguments)};b.wbg.__wbg_done_506b44765ba84b9c=(a=>{const b=c(a).done;return b});b.wbg.__wbg_value_31485d8770eb06ab=(a=>{const b=c(a).value;return k(b)});b.wbg.__wbg_iterator_364187e1ee96b750=(()=>{const a=Symbol.iterator;return k(a)});b.wbg.__wbg_get_5027b32da70f39b1=function(){return C(((a,b)=>{const d=a4.get(c(a),c(b));return k(d)}),arguments)};b.wbg.__wbg_call_a79f1973a4f07d5e=function(){return C(((a,b)=>{const d=c(a).call(c(b));return k(d)}),arguments)};b.wbg.__wbg_new_87d841e70661f6e9=(()=>{const a=new a5();return k(a)});b.wbg.__wbg_self_086b5302bcafb962=function(){return C((()=>{const a=self.self;return k(a)}),arguments)};b.wbg.__wbg_window_132fa5d7546f1de5=function(){return C((()=>{const a=window.window;return k(a)}),arguments)};b.wbg.__wbg_globalThis_e5f801a37ad7d07b=function(){return C((()=>{const a=globalThis.globalThis;return k(a)}),arguments)};b.wbg.__wbg_global_f9a61fce4af6b7c1=function(){return C((()=>{const a=global.global;return k(a)}),arguments)};b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===S;return b});b.wbg.__wbg_from_a663e01d8dab8e44=(a=>{const b=Q.from(c(a));return k(b)});b.wbg.__wbg_new_3a66822ed076951c=((a,b)=>{const c=new W(j(a,b));return k(c)});b.wbg.__wbg_call_f6a2bc58c19c53c6=function(){return C(((a,b,d)=>{const e=c(a).call(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_is_a5728dbfb61c82cd=((a,b)=>{const d=a5.is(c(a),c(b));return d});b.wbg.__wbg_new_1d93771b84541aa5=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=X;try{return D(d,c.b,a,b)}finally{c.a=d}};const e=new a6(d);return k(e)}finally{c.a=c.b=X}});b.wbg.__wbg_resolve_97ecd55ee839391b=(a=>{const b=a6.resolve(c(a));return k(b)});b.wbg.__wbg_then_7aeb7c5f1536640f=((a,b)=>{const d=c(a).then(c(b));return k(d)});b.wbg.__wbg_then_5842e4e97f7beace=((a,b,d)=>{const e=c(a).then(c(b),c(d));return k(e)});b.wbg.__wbg_buffer_5d1b598a01b41a42=(a=>{const b=c(a).buffer;return k(b)});b.wbg.__wbg_newwithbyteoffsetandlength_d695c7957788f922=((a,b,d)=>{const e=new Y(c(a),b>>>X,d>>>X);return k(e)});b.wbg.__wbg_new_ace717933ad7117f=(a=>{const b=new Y(c(a));return k(b)});b.wbg.__wbg_set_74906aa30864df5a=((a,b,d)=>{c(a).set(c(b),d>>>X)});b.wbg.__wbg_length_f0764416ba5bb237=(a=>{const b=c(a).length;return b});b.wbg.__wbg_stringify_daa6661e90c04140=function(){return C((a=>{const b=a1(c(a));return k(b)}),arguments)};b.wbg.__wbg_has_a2919659b7b645b3=function(){return C(((a,b)=>{const d=a4.has(c(a),c(b));return d}),arguments)};b.wbg.__wbg_set_37a50e901587b477=function(){return C(((a,b,d)=>{const e=a4.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=s(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a2+ Z]=g;r()[b/a2+ X]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new W(j(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return k(b)});b.wbg.__wbindgen_closure_wrapper972=((a,b,c)=>{const d=t(a,b,478,w);return k(d)});b.wbg.__wbindgen_closure_wrapper1365=((a,b,c)=>{const d=x(a,b,628,y);return k(d)});return b});var K=(async(a,b)=>{if(typeof Response===_&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===_){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var k=(a=>{if(d===b.length)b.push(b.length+ Z);const c=d;d=b[c];b[c]=a;return c});var P=(async(b)=>{if(a!==S)return a;if(typeof b===U){b=new URL(`rust-study-front-f6f53797befef4ea_bg.wasm`,import.meta.url)};const c=L();if(typeof b===$||typeof Request===_&&b instanceof Request||typeof URL===_&&b instanceof URL){b=fetch(b)};M(c);const {instance:d,module:e}=await K(await b,c);return N(d,e)});var o=((a,b,c)=>{if(c===S){const c=m.encode(a);const d=b(c.length,Z)>>>X;i().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,Z)>>>X;const f=i();let g=X;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==X){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,Z)>>>X;const b=i().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written};l=g;return e});var M=((a,b)=>{});var e=(a=>{if(a<132)return;b[a]=d;d=a});var i=(()=>{if(h===T||h.byteLength===X){h=new Y(a.memory.buffer)};return h});var c=(a=>b[a]);var w=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_ref__h7b65436d631b68fd(c,d,v(e))}finally{b[u++]=S}});var r=(()=>{if(q===T||q.byteLength===X){q=new Int32Array(a.memory.buffer)};return q});var v=(a=>{if(u==Z)throw new W(`out of js stack`);b[--u]=a;return u});var D=((b,c,d,e)=>{a.wasm_bindgen__convert__closures__invoke2_mut__h7d5a631ea9478361(b,c,k(d),k(e))});function C(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(k(b))}}var B=((a,b)=>{a=a>>>X;const c=A();const d=c.subarray(a/a2,a/a2+ b);const e=[];for(let a=X;a<d.length;a++){e.push(f(d[a]))};return e});let a;const b=new Q(R).fill(S);b.push(S,T,!0,!1);let d=b.length;const g=typeof TextDecoder!==U?new TextDecoder(V,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw W(`TextDecoder not available`)}};if(typeof TextDecoder!==U){g.decode()};let h=T;let l=X;const m=typeof TextEncoder!==U?new TextEncoder(V):{encode:()=>{throw W(`TextEncoder not available`)}};const n=typeof m.encodeInto===_?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=T;let u=R;let z=T;class E{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=X;return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingbytesource_free(b)}type(){let b;let c;try{const f=a.__wbindgen_add_to_stack_pointer(-a3);a.intounderlyingbytesource_type(f,this.__wbg_ptr);var d=r()[f/a2+ X];var e=r()[f/a2+ Z];b=d;c=e;return j(d,e)}finally{a.__wbindgen_add_to_stack_pointer(a3);a.__wbindgen_free(b,c,Z)}}autoAllocateChunkSize(){const b=a.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return b>>>X}start(b){a.intounderlyingbytesource_start(this.__wbg_ptr,k(b))}pull(b){const c=a.intounderlyingbytesource_pull(this.__wbg_ptr,k(b));return f(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingbytesource_cancel(b)}}class F{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=X;return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsink_free(b)}write(b){const c=a.intounderlyingsink_write(this.__wbg_ptr,k(b));return f(c)}close(){const b=this.__destroy_into_raw();const c=a.intounderlyingsink_close(b);return f(c)}abort(b){const c=this.__destroy_into_raw();const d=a.intounderlyingsink_abort(c,k(b));return f(d)}}class G{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=X;return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsource_free(b)}pull(b){const c=a.intounderlyingsource_pull(this.__wbg_ptr,k(b));return f(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingsource_cancel(b)}}class H{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=X;return a}free(){const b=this.__destroy_into_raw();a.__wbg_pipeoptions_free(b)}preventClose(){const b=a.pipeoptions_preventClose(this.__wbg_ptr);return b!==X}preventCancel(){const b=a.pipeoptions_preventCancel(this.__wbg_ptr);return b!==X}preventAbort(){const b=a.pipeoptions_preventAbort(this.__wbg_ptr);return b!==X}signal(){const b=a.pipeoptions_signal(this.__wbg_ptr);return f(b)}}class I{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=X;return a}free(){const b=this.__destroy_into_raw();a.__wbg_queuingstrategy_free(b)}highWaterMark(){const b=a.queuingstrategy_highWaterMark(this.__wbg_ptr);return b}}class J{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=X;return a}free(){const b=this.__destroy_into_raw();a.__wbg_readablestreamgetreaderoptions_free(b)}mode(){const b=a.readablestreamgetreaderoptions_mode(this.__wbg_ptr);return f(b)}}export default P;export{E as IntoUnderlyingByteSource,F as IntoUnderlyingSink,G as IntoUnderlyingSource,H as PipeOptions,I as QueuingStrategy,J as ReadableStreamGetReaderOptions,O as initSync}