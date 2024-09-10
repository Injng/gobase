var C=Object.defineProperty;var j=(e,t,n)=>t in e?C(e,t,{enumerable:!0,configurable:!0,writable:!0,value:n}):e[t]=n;var p=(e,t,n)=>(j(e,typeof t!="symbol"?t+"":t,n),n);import{r as h,n as y,f as w,h as B,i as E,j as I,k as b,l as D,m as L,p as N,q as P,v as T,w as q}from"./scheduler.CfYsyzRB.js";let $=!1;function H(){$=!0}function M(){$=!1}function O(e,t,n,r){for(;e<t;){const l=e+(t-e>>1);n(l)<=r?e=l+1:t=l}return e}function z(e){if(e.hydrate_init)return;e.hydrate_init=!0;let t=e.childNodes;if(e.nodeName==="HEAD"){const i=[];for(let a=0;a<t.length;a++){const o=t[a];o.claim_order!==void 0&&i.push(o)}t=i}const n=new Int32Array(t.length+1),r=new Int32Array(t.length);n[0]=-1;let l=0;for(let i=0;i<t.length;i++){const a=t[i].claim_order,o=(l>0&&t[n[l]].claim_order<=a?l+1:O(1,l,d=>t[n[d]].claim_order,a))-1;r[i]=n[o]+1;const u=o+1;n[u]=i,l=Math.max(u,l)}const c=[],s=[];let f=t.length-1;for(let i=n[l]+1;i!=0;i=r[i-1]){for(c.push(t[i-1]);f>=i;f--)s.push(t[f]);f--}for(;f>=0;f--)s.push(t[f]);c.reverse(),s.sort((i,a)=>i.claim_order-a.claim_order);for(let i=0,a=0;i<s.length;i++){for(;a<c.length&&s[i].claim_order>=c[a].claim_order;)a++;const o=a<c.length?c[a]:null;e.insertBefore(s[i],o)}}function R(e,t){if($){for(z(e),(e.actual_end_child===void 0||e.actual_end_child!==null&&e.actual_end_child.parentNode!==e)&&(e.actual_end_child=e.firstChild);e.actual_end_child!==null&&e.actual_end_child.claim_order===void 0;)e.actual_end_child=e.actual_end_child.nextSibling;t!==e.actual_end_child?(t.claim_order!==void 0||t.parentNode!==e)&&e.insertBefore(t,e.actual_end_child):e.actual_end_child=t.nextSibling}else(t.parentNode!==e||t.nextSibling!==null)&&e.appendChild(t)}function ne(e,t,n){$&&!n?R(e,t):(t.parentNode!==e||t.nextSibling!=n)&&e.insertBefore(t,n||null)}function U(e){e.parentNode&&e.parentNode.removeChild(e)}function ie(e,t){for(let n=0;n<e.length;n+=1)e[n]&&e[n].d(t)}function V(e){return document.createElement(e)}function W(e){return document.createElementNS("http://www.w3.org/2000/svg",e)}function x(e){return document.createTextNode(e)}function re(){return x(" ")}function ae(){return x("")}function se(e,t,n,r){return e.addEventListener(t,n,r),()=>e.removeEventListener(t,n,r)}function le(e){return function(t){return t.preventDefault(),e.call(this,t)}}function F(e,t,n){n==null?e.removeAttribute(t):e.getAttribute(t)!==n&&e.setAttribute(t,n)}function fe(e,t){for(const n in t)F(e,n,t[n])}function ce(e){return e.dataset.svelteH}function G(e){return Array.from(e.childNodes)}function J(e){e.claim_info===void 0&&(e.claim_info={last_index:0,total_claimed:0})}function S(e,t,n,r,l=!1){J(e);const c=(()=>{for(let s=e.claim_info.last_index;s<e.length;s++){const f=e[s];if(t(f)){const i=n(f);return i===void 0?e.splice(s,1):e[s]=i,l||(e.claim_info.last_index=s),f}}for(let s=e.claim_info.last_index-1;s>=0;s--){const f=e[s];if(t(f)){const i=n(f);return i===void 0?e.splice(s,1):e[s]=i,l?i===void 0&&e.claim_info.last_index--:e.claim_info.last_index=s,f}}return r()})();return c.claim_order=e.claim_info.total_claimed,e.claim_info.total_claimed+=1,c}function A(e,t,n,r){return S(e,l=>l.nodeName===t,l=>{const c=[];for(let s=0;s<l.attributes.length;s++){const f=l.attributes[s];n[f.name]||c.push(f.name)}c.forEach(s=>l.removeAttribute(s))},()=>r(t))}function ue(e,t,n){return A(e,t,n,V)}function oe(e,t,n){return A(e,t,n,W)}function K(e,t){return S(e,n=>n.nodeType===3,n=>{const r=""+t;if(n.data.startsWith(r)){if(n.data.length!==r.length)return n.splitText(r.length)}else n.data=r},()=>x(t),!0)}function _e(e){return K(e," ")}function de(e,t){t=""+t,e.data!==t&&(e.data=t)}function me(e,t,n,r){n==null?e.style.removeProperty(t):e.style.setProperty(t,n,"")}function he(e,t){return new e(t)}const m=new Set;let _;function $e(){_={r:0,c:[],p:_}}function pe(){_.r||h(_.c),_=_.p}function Q(e,t){e&&e.i&&(m.delete(e),e.i(t))}function ye(e,t,n,r){if(e&&e.o){if(m.has(e))return;m.add(e),_.c.push(()=>{m.delete(e),r&&(n&&e.d(1),r())}),e.o(t)}else r&&r()}function xe(e){e&&e.c()}function ge(e,t){e&&e.l(t)}function X(e,t,n){const{fragment:r,after_update:l}=e.$$;r&&r.m(t,n),b(()=>{const c=e.$$.on_mount.map(P).filter(E);e.$$.on_destroy?e.$$.on_destroy.push(...c):h(c),e.$$.on_mount=[]}),l.forEach(b)}function Y(e,t){const n=e.$$;n.fragment!==null&&(D(n.after_update),h(n.on_destroy),n.fragment&&n.fragment.d(t),n.on_destroy=n.fragment=null,n.ctx=[])}function Z(e,t){e.$$.dirty[0]===-1&&(T.push(e),q(),e.$$.dirty.fill(0)),e.$$.dirty[t/31|0]|=1<<t%31}function ve(e,t,n,r,l,c,s=null,f=[-1]){const i=L;N(e);const a=e.$$={fragment:null,ctx:[],props:c,update:y,not_equal:l,bound:w(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(t.context||(i?i.$$.context:[])),callbacks:w(),dirty:f,skip_bound:!1,root:t.target||i.$$.root};s&&s(a.root);let o=!1;if(a.ctx=n?n(e,t.props||{},(u,d,...g)=>{const v=g.length?g[0]:d;return a.ctx&&l(a.ctx[u],a.ctx[u]=v)&&(!a.skip_bound&&a.bound[u]&&a.bound[u](v),o&&Z(e,u)),d}):[],a.update(),o=!0,h(a.before_update),a.fragment=r?r(a.ctx):!1,t.target){if(t.hydrate){H();const u=G(t.target);a.fragment&&a.fragment.l(u),u.forEach(U)}else a.fragment&&a.fragment.c();t.intro&&Q(e.$$.fragment),X(e,t.target,t.anchor),M(),B()}N(i)}class we{constructor(){p(this,"$$");p(this,"$$set")}$destroy(){Y(this,1),this.$destroy=y}$on(t,n){if(!E(n))return y;const r=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return r.push(n),()=>{const l=r.indexOf(n);l!==-1&&r.splice(l,1)}}$set(t){this.$$set&&!I(t)&&(this.$$.skip_bound=!0,this.$$set(t),this.$$.skip_bound=!1)}}const k="4";typeof window<"u"&&(window.__svelte||(window.__svelte={v:new Set})).v.add(k);export{fe as A,ie as B,ce as C,se as D,le as E,we as S,G as a,K as b,ue as c,U as d,V as e,_e as f,ne as g,R as h,ve as i,de as j,Q as k,ye as l,ae as m,pe as n,F as o,me as p,$e as q,he as r,re as s,x as t,xe as u,ge as v,X as w,Y as x,W as y,oe as z};