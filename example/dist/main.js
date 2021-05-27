!(function (modules) {
  var installedModules = {};
  function __rustpack_require__(moduleId) {
    if (installedModules[moduleId]) {
      return installedModules[moduleId].exports;
    }
    var module = (installedModules[moduleId] = {
      exports: {},
    });
    modules[moduleId].call(
      module.exports,
      module,
      module.exports,
      __rustpack_require__
    );
    return module.exports;
  }
  // 入口
  return __rustpack_require__("./index.js");
})({"./a.js":function(module, exports, __rustpack_require__){
module.exports = (message) => {
  console.log(message);
};
},
"./index.js":function(module, exports, __rustpack_require__){
const Index = __rustpack_require__("./a.js");
Index("Hello world!");
},
});