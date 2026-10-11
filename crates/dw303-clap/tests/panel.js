// Dependency-free probe of DW-303's custom wiring; RAVET widgets are stubbed.
// Run: node crates/dw303-clap/tests/panel.js
const fs=require('fs'),vm=require('vm'),assert=require('assert');
const html=fs.readFileSync('editor/index.html','utf8'),nodes=new Map(),messages=[];
class Node {
  constructor(id){this.id=id;this.listeners={};this.dataset={};this.attrs={};this.style={setProperty:(k,v)=>this.attrs[k]=v};}
  addEventListener(k,f){(this.listeners[k]??=[]).push(f);}
  setAttribute(k,v){this.attrs[k]=v;}
  focus(){} setPointerCapture(){}
  emit(type,extra={}){for(const f of this.listeners[type]||[])f({button:0,pointerId:1,detail:0,preventDefault(){},...extra});}
}
for(const match of html.matchAll(/id="([^"]+)"/g))nodes.set(match[1],new Node(match[1]));
const hits=[...html.matchAll(/<button id="([^"]+)" class="hit"/g)].map(m=>nodes.get(m[1]));
const lamps=[...nodes.values()].filter(n=>n.id.startsWith('led-'));
class Knob {constructor(node,number,spec,post){this.node=node;this.current=spec.value;this.spec=spec;this.post=post;}paint(v){this.current=v;}update(v){this.paint(v);}commit(v){this.paint(v);this.post({type:'edit',value:v});}}
class Simple {constructor(){}update(){}}
const context=vm.createContext({document:{getElementById:id=>nodes.get(id),querySelectorAll:q=>q==='.hit'?hits:lamps},innerWidth:1227,innerHeight:589,addEventListener(){},setInterval(){},setTimeout:f=>f(),performance:{now:()=>0},RavetControls:{Knob,Toggle:Simple,Choice:Simple},webkit:{messageHandlers:{ravet:{postMessage:m=>messages.push(m)}}}});
context.window=context;
vm.runInContext(html.slice(html.lastIndexOf('<script>')+8,html.lastIndexOf('</script>')),context);
function evaluate(code){return vm.runInContext(code,context);}
function edge(id){nodes.get(id).emit('pointerdown');nodes.get(id).emit('pointerup');}
function actions(){return messages.splice(0).map(m=>[m.action,m.key,m.edge,m.value]);}
edge('run-stop');assert.deepStrictEqual(actions(),[['button','run-stop','press',undefined],['button','run-stop','release',undefined]]);
nodes.get('key-c').emit('keydown',{key:'Enter'});nodes.get('key-c').emit('keyup',{key:'Enter'});assert.deepStrictEqual(actions().map(a=>a.slice(0,3)),[['button','key-c','press'],['button','key-c','release']]);
edge('function');assert.equal(actions()[0][1],'clear');edge('clear');assert.equal(actions()[0][1],'time-mode');edge('time-mode');assert.equal(actions()[0][1],'key-back');
evaluate('editMode=0');edge('key-slide');assert.deepStrictEqual(actions()[0],['section',undefined,undefined,0]);edge('key-back');assert.deepStrictEqual(actions()[0],['section',undefined,undefined,1]);
nodes.get('key-down').emit('pointerdown');edge('key-up');edge('key-accent');nodes.get('key-down').emit('pointerup');assert.deepStrictEqual(actions().map(a=>a[1]),['function','key-step','key-step','key-triplet','key-triplet','function']);
nodes.get('key-c').emit('pointerdown');nodes.get('key-c').emit('pointercancel');nodes.get('key-c').emit('lostpointercapture');assert.equal(actions().length,2);
for(const [name,count] of [['mode',4],['pattern-group',7]]){for(let value=0;value<count;value++){evaluate(`selectors.get('${name}').commit(${value})`);assert.deepStrictEqual(actions()[0],[name,undefined,undefined,value]);}}
evaluate('window.ravetUpdate({params:{12:1},leds:{run:1,"step-1":2},selection:{mode:3,editMode:1,group:0,track:0,section:"A",pattern:1,step:4}})');assert.equal(nodes.get('led-run').dataset.on,'1');assert.equal(nodes.get('led-step-1').dataset.on,'1');assert(nodes.get('status').textContent.includes('Pattern Write'));evaluate('window.ravetUpdate({leds:{}})');assert(lamps.every(l=>l.dataset.on==='0'));
console.log('Panel wiring checks passed: edges, artwork bindings, sections, secondary keys, selectors, LEDs/status.');
