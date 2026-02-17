const token = location.hash.substring(2);
const signupLink = document.getElementById("xmpp-signup-link");

if (signupLink) {
  signupLink.href = `xmpp:lark@floof.chat?roster;preauth=${token};ibr=y`;
}
