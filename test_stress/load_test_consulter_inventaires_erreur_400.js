import http from 'k6/http';
import { sleep } from 'k6';

export let options = {
  stages: [
    { duration: '1m', target: 100 },
    { duration: '1m', target: 500 },
    { duration: '1m', target: 800 },
    { duration: '1m', target: 700 },
    { duration: '1m', target: 600 },
  ],
};

export default function () {
  let inventaire = "11";
  const url = `http://127.0.0.1:8000/api/inventaires/${inventaire}`;
  let res = http.get(url);

  if (res.status !== 200) {
    console.error(`Erreur HTTP ${res.status}`);
  }

  sleep(1);
}