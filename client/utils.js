export const log = (msg) => (res) => {
  console.log(`${msg}: ${S.show(res)}`);
  return res;
};
