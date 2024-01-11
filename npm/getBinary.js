const { Binary } = require('binary-install');
const os = require('os');

function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  const platformInfo = {
    arch: '',
    type: '',
  };

  switch(type) {
    case 'Windows_NT': {
      platformInfo.type = 'pc-windows-msvc';
    } break;
    case 'Darwin': {
      platformInfo.type = 'apple-darwin';
    } break;
    case 'Linux': {
      platformInfo.type = 'linux-gnu';
    } break;
  }

  if (['arm', 'arm64'].includes(arch)) {
    platformInfo.arch = 'aarch64';
  } else {
    platformInfo.arch = 'x86_64';
  }

  return platformInfo;
}

function getBinary() {
  const platform = getPlatform();
  const version = require('../package.json').version;
  const url = `https://github.com/wisdomstar94/onece-exec/releases/download/v${version}/onece-exec-${platform.arch}-${platform.type}.tar.gz`;
  const name = 'onece-exec';
  return new Binary(name, url);
}

module.exports = getBinary;