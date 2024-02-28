export interface BreadCrumb {
  label: string,
  href: string,
};

export function generateBreadcrumbs(path: string, replacements: {[name: string]: string} = {}): BreadCrumb[] {
  const crumbs = path.split('/');
  return crumbs.map((c, i) => {
    const label = replacements[c] === undefined ? c.replace('-', ' ') : replacements[c]
    return {
      label,
      href: crumbs.slice(0, i+1).join('/'),
    };
  }).slice(1, -1);
}