let lastActiveElement: Element | null = null;

export function setRenderingOverlay(
    isVisible: boolean,
    message = 'Rendering... UI is temporarily disabled.',
    container: HTMLElement | null = document.getElementById('app') as HTMLElement | null ?? document.body,
): void {
    const overlay = document.getElementById('rendering-overlay') as HTMLElement | null;
    if (!overlay) return;

    const text = document.getElementById('rendering-overlay-text');
    if (text) {
        text.textContent = message;
    }

    if (!overlay.hasAttribute('tabindex')) {
        overlay.tabIndex = -1;
    }
    if (!overlay.hasAttribute('role')) {
        overlay.setAttribute('role', 'status');
    }

    if (isVisible) {
        if (document.activeElement) {
            lastActiveElement = document.activeElement;
        }

        if (container) {
            container.setAttribute('inert', '');
            container.setAttribute('aria-hidden', 'true');
        }

        overlay.style.display = 'flex';

        const focusTarget =
            (overlay.querySelector<HTMLElement>(
                'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
            ) as HTMLElement | null) || overlay;
        setTimeout(() => {
            try {
                focusTarget.focus();
            } catch {
                /* ignore */
            }
        }, 0);
    } else {
        if (container) {
            container.removeAttribute('inert');
            container.removeAttribute('aria-hidden');
        }

        overlay.style.display = 'none';

        if (lastActiveElement instanceof HTMLElement) {
            const elementToFocus = lastActiveElement;
            lastActiveElement = null;
            if (document.contains(elementToFocus)) {
                try {
                    elementToFocus.focus();
                } catch {
                    /* ignore */
                }
            }
        } else {
            lastActiveElement = null;
        }
    }
}
