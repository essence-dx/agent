import { useState, useEffect } from 'react';
import { Outlet, useLocation } from 'react-router-dom';
import Sidebar from '@/components/layout/Sidebar';
import Header from '@/components/layout/Header';
import ReloadBanner from '@/components/layout/ReloadBanner';
import UnsavedChangesBanner from '@/components/layout/UnsavedChangesBanner';
import { ErrorBoundary } from '@/App';
import { getCompatStorageItem, safeLocalStorage, setCompatStorageItem } from '@/lib/compatStorage';

const SIDEBAR_COLLAPSED_KEY = 'dx_agents-sidebar-collapsed';
const LEGACY_SIDEBAR_COLLAPSED_KEYS = ['zeroclaw-sidebar-collapsed'] as const;

export default function Layout() {
  const { pathname } = useLocation();
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [collapsed, setCollapsed] = useState(() => {
    try {
      return getCompatStorageItem(safeLocalStorage(), SIDEBAR_COLLAPSED_KEY, LEGACY_SIDEBAR_COLLAPSED_KEYS) === 'true';
    } catch {
      return false;
    }
  });

  // Close sidebar on route change (mobile navigation)
  useEffect(() => {
    setSidebarOpen(false);
  }, [pathname]);

  // Persist collapsed state
  useEffect(() => {
    try {
      setCompatStorageItem(safeLocalStorage(), SIDEBAR_COLLAPSED_KEY, LEGACY_SIDEBAR_COLLAPSED_KEYS, String(collapsed));
    } catch {
      // localStorage may not be available
    }
  }, [collapsed]);

  return (
    <div className="min-h-screen text-white" style={{ background: 'var(--pc-bg-base)' }}>
      {/* Fixed sidebar */}
      <Sidebar open={sidebarOpen} onClose={() => setSidebarOpen(false)} collapsed={collapsed} />

      {/* Main area — offset by sidebar width on desktop, full-width on mobile */}
      <div
        className={`
          flex flex-col flex-1 min-w-0 h-screen transition-all duration-300 ease-in-out
          ${collapsed ? 'md:ml-14' : 'md:ml-60'}
          ml-0
        `}
      >
        <Header
          onMenuToggle={() => setSidebarOpen((v) => !v)}
          onCollapseToggle={() => setCollapsed((c) => !c)}
          collapsed={collapsed}
        />
        <ReloadBanner />
        <UnsavedChangesBanner />

        {/* Page content — ErrorBoundary keyed by the first path segment
            so the boundary resets when the user navigates between pages
            (e.g. /agent → /config), but stays mounted across param-only
            changes within a page (e.g. /config/providers → /config/browser).
            Keying on the full pathname remounted the entire route tree
            on every section click and reset scroll/state. */}
        <main className="flex-1 overflow-y-auto min-h-0">
          <ErrorBoundary key={pathname.split('/')[1] ?? ''}>
            <Outlet />
          </ErrorBoundary>
        </main>
      </div>
    </div>
  );
}
