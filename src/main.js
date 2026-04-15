document.addEventListener('DOMContentLoaded', () => {
    if (window.__TAURI__) {
        const { invoke } = window.__TAURI__.tauri;

        // --- CACHE DES ÉLÉMENTS DOM ---
        const cpuElem = document.getElementById('cpu-val');
        const cpuBar = document.getElementById('cpu-bar');
        const ramElem = document.getElementById('ram-val');
        const ramBar = document.getElementById('ram-bar');
        const optiBtn = document.getElementById('opti-btn');

        // --- ÉTAT LOCAL POUR DÉTECTION DE CHANGEMENT ---
        let lastCpu = -1;
        let lastRamUsed = -1;

        // --- FONCTION DE MISE À JOUR DES STATS ---
        async function updateStats() {
            try {
                const stats = await invoke('get_stats');
                
                const cpu = Math.round(stats.cpu || 0);
                const used = stats.ram_used || 0;
                const total = stats.ram_total || 1;

                // Mise à jour CPU seulement si changement
                if (cpu !== lastCpu) {
                    if (cpuElem) cpuElem.textContent = `${cpu}%`;
                    if (cpuBar) cpuBar.style.width = `${cpu}%`;
                    lastCpu = cpu;
                }

                // Mise à jour RAM seulement si changement
                if (used !== lastRamUsed) {
                    if (ramElem) ramElem.textContent = `${used} / ${total} MB`;
                    if (ramBar) {
                        const ramPercent = (used / total) * 100;
                        ramBar.style.width = `${ramPercent}%`;
                    }
                    lastRamUsed = used;
                }

            } catch (error) {
                console.error("Erreur de communication avec Rust:", error);
            }
        }

        const cacheBtn = document.getElementById('cache-btn');

        // --- GESTION DU BOUTON OPTIMISER RAM ---
        if (optiBtn) {
            optiBtn.addEventListener('click', async () => {
                const originalText = optiBtn.textContent;
                optiBtn.textContent = "CHARGEMENT...";
                optiBtn.disabled = true;

                try {
                    const result = await invoke('optimize_ram');
                    optiBtn.textContent = result;
                    optiBtn.style.background = "#28a745"; 
                } catch (err) {
                    console.error(err);
                    optiBtn.textContent = "ERREUR";
                }

                setTimeout(() => {
                    optiBtn.textContent = originalText;
                    optiBtn.style.background = ""; 
                    optiBtn.disabled = false;
                }, 3000);
            });
        }

        // --- GESTION DU BOUTON VIDER CACHE ---
        if (cacheBtn) {
            cacheBtn.addEventListener('click', async () => {
                const originalText = cacheBtn.textContent;
                cacheBtn.textContent = "NETTOYAGE...";
                cacheBtn.disabled = true;

                try {
                    const result = await invoke('clean_cache');
                    cacheBtn.textContent = result;
                    cacheBtn.style.borderColor = "#28a745";
                    cacheBtn.style.color = "#28a745";
                } catch (err) {
                    console.error(err);
                    cacheBtn.textContent = "ERREUR";
                }

                setTimeout(() => {
                    cacheBtn.textContent = originalText;
                    cacheBtn.style.borderColor = "";
                    cacheBtn.style.color = "";
                    cacheBtn.disabled = false;
                }, 3000);
            });
        }

        setInterval(updateStats, 1000);
        updateStats();
    } else {
        console.error("L'API Tauri est introuvable.");
    }
});