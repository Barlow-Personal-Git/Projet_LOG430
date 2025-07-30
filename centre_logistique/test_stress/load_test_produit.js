import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
  stages: [
    { duration: '1m', target: 1000 },
    { duration: '1m', target: 2000 },
    { duration: '1m', target: 3000 },
    { duration: '1m', target: 2000 },
    { duration: '1m', target: 1000 },
  ],
};

export default function () {
    const payload = JSON.stringify({
        nom: "Test produit",
        prix: 123.45,
        description: "Mise à jour"
    });

    const params = {
        headers: { 'Content-Type': 'application/json' },
    };

    let res = http.put('http://127.0.0.1:8000/api/produits/1', payload, params);

    check(res, {
        'status is 200': (r) => r.status === 200,
    });

    sleep(1);
}
