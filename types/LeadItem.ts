export interface LeadItem {
    id: string;
    name: string;
    website: string;
    phone: string;
    address: string;
    has_website: boolean;
    has_phone: boolean;
    total_reviews: string;
    ratings: string;
    url: string;
    owner_name: string;
    call_completed: boolean;
    call_successful: boolean;
    location: string;
    status: 'pending' | 'processing' | 'success' | 'failed';
}
