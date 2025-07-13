import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
  stages: [
    { duration: '1m', target: 10 },
    { duration: '1m', target: 20 },
    { duration: '1m', target: 30 },
    { duration: '1m', target: 20 },
    { duration: '1m', target: 10 },
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

    let res = http.put('http://localhost:8000/produits/1', payload, params);

    check(res, {
        'status is 200': (r) => r.status === 200,
    });

    sleep(1);
}
