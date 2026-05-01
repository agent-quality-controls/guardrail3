import recommended from "./configs/recommended.js";
import plugin, { rules } from "./plugin.js";

export { recommended, rules };
export default { ...plugin, configs: { recommended } };
