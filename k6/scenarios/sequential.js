import http from 'k6/http';

export const options = {
  vus: 1,
};

export default function () {
  const url = `${__ENV.GATEWAY_URL}/${__ENV.LANG_CASE}`;

  for(let iteration = 0; iteration < 1000; iteration++) {
    const payload = JSON.stringify({
      'iteration': iteration,
    });

    const params = {
      headers: { 'Content-Type': 'application/json' },
    };

    http.post(url, payload, params);
  }
}
