const { createProxyMiddleware } = require('http-proxy-middleware');

module.exports = function (app) {
    app.use(
        createProxyMiddleware('/api', {
            target: 'http://127.0.0.1:5000',
        })
    );

    app.use(
        createProxyMiddleware('/socket.io', {
            target: 'http://127.0.0.1:5000',
            ws: true,
        })
    );
};
