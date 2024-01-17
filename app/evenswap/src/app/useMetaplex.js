const DEFAULT_CONTEXT = {
    metaplex: null,
};

export const MetaplexContext = createContext(DEFAULT_CONTEXT);

export function useMetaplex() {
    return useContext(MetaplexContext);
}