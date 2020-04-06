module.exports = async () => {
  console.log('running jest.afterall.js stop');
  global.api.kill();
  global.api.unref();
};
