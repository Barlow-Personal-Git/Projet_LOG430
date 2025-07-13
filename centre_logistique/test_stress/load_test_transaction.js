import http from 'k6/http';
import { sleep } from 'k6';

export let options = {
  stages: [
    { duration: '1m', target: 100 },
    { duration: '1m', target: 300 },
    { duration: '1m', target: 600 },
    { duration: '1m', target: 300 },
    { duration: '1m', target: 100 },
  ],
};

export default function () {
  
  const url = `http://127.0.0.1:8000/api/tendances_hebdomadaires`;
  let res = http.get(url);

  if (res.status !== 200) {
    console.error(`Erreur HTTP ${res.status}`);
  }

  sleep(1);
}
