document.getElementById('startBtn').addEventListener('click', () => {
    document.getElementById('verificationSection').style.display = 'block';
    document.getElementById('startBtn').style.display = 'none';
});

document.getElementById('verify-button').addEventListener('click', verifyProduct);
document.getElementById('generateQRBtn').addEventListener('click', generateQRCode);

let lastVerifiedProductId = '';

async function verifyProduct() {
    const productId = document.getElementById('productIdInput').value.trim();
    const resultEl = document.getElementById('result');
    resultEl.textContent = '';
    clearQRCode();

    if (!productId) {
        alert('Please enter a product ID');
        return;
    }

    try {
        const res = await fetch(`/api/product/${encodeURIComponent(productId)}`);
        if (!res.ok) {
            if (res.status === 404) {
                resultEl.textContent = 'Product not found';
            } else {
                resultEl.textContent = 'Error checking product';
            }
            lastVerifiedProductId = '';
            return;
        }

        const data = await res.json();
        lastVerifiedProductId = productId;

        const authenticity = data.authentic ? '✅ Authentic' : '❌ Fake';
        resultEl.textContent = `${data.productName} by ${data.manufacturer} (${data.origin}) - ${authenticity}`;
    } catch (err) {
        resultEl.textContent = 'Network error';
        lastVerifiedProductId = '';
    }
}

function clearQRCode() {
    const qrContainer = document.getElementById('qrcode');
    qrContainer.innerHTML = '';
}

function generateQRCode() {
    clearQRCode();
    if (!lastVerifiedProductId) {
        alert('Please verify a valid product ID first!');
        return;
    }
    new QRCode(document.getElementById('qrcode'), {
        text: lastVerifiedProductId,
        width: 180,
        height: 180,
    });
}
